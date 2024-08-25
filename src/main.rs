use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::anyhow;
use futures::StreamExt;
use librespot::core::cache::Cache;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::discovery::{Credentials, DeviceType};
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::NoOpVolume;
use librespot::playback::player::Player;
use librespot::{core::config::SessionConfig, discovery::Discovery};
use sha1::{Digest, Sha1};

mod clap;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let command = clap::args::cli().get_matches();

    match command.subcommand() {
        Some(("discover", _)) => {
            // Handle the 'discover' subcommand
            println!("Discovering Spotify Connect devices");
            discover().await?;
        }
        Some(("play", sub_m)) => {
            let credentials = fetch_credentials().await?;
            let session_config = SessionConfig::default();

            // Handle the 'play' subcommand
            if let Some(track) = sub_m.get_one::<String>("track") {
                let (session, _) = Session::connect(session_config, credentials, None, false)
                    .await
                    .expect("Failed to connect to Spotify");

                play_track(session, track).await?;
            } else {
                return Err(anyhow!("Track ID is required for the play command"));
            }
        }
        _ => {
            return Err(anyhow!("No valid subcommand provided"));
        }
    }

    Ok(())
}

async fn fetch_credentials() -> Result<Credentials, anyhow::Error> {
    let path = Path::new("cache/credentials.json");

    if !path.exists() {
        return Err(anyhow::anyhow!(format!(
            "File {} does not exist.",
            path.display()
        )));
    }

    let file =
        File::open(path).map_err(|e| anyhow::anyhow!(format!("Failed to open file: {}", e)))?;

    let reader = BufReader::new(file);
    let credentials: Credentials = serde_json::from_reader(reader)
        .map_err(|e| anyhow::anyhow!(format!("Failed to parse json: {}", e)))?;

    Ok(credentials)
}

async fn discover() -> Result<(), anyhow::Error> {
    let name = "ConnectExample";
    let device_id = hex::encode(Sha1::digest(name.as_bytes()));

    let mut discovery = Discovery::builder(device_id)
        .device_type(DeviceType::Speaker)
        .launch()
        .unwrap();

    println!("Searching for Spotify Connect devices");
    while let Some(credentials) = discovery.next().await {
        let session_config = SessionConfig::default();

        let credentials_path = Some("cache");
        let volume_path = Some("cache");
        let audio_path = Some("cache");
        let size_limit = Some(1024 * 1024 * 1024);

        let cache =
            Some(Cache::new(credentials_path, volume_path, audio_path, size_limit).unwrap());

        let (session, _) = Session::connect(session_config, credentials, cache, true)
            .await
            .expect("Failed to connect to Spotify");

        println!("Found device: {}, saved credentials", session.device_id());
        break;
    }

    Ok(())
}

async fn play_track(session: Session, track_uri: &str) -> Result<(), anyhow::Error> {
    println!("Start to play track {}", track_uri);

    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    let track_id = SpotifyId::from_uri(&track_uri).unwrap();
    let backend = audio_backend::find(None).unwrap();

    let (mut player, _) = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });

    player.load(track_id, true, 0);
    println!("Playing...");
    player.await_end_of_track().await;
    println!("Done");

    Ok(())
}

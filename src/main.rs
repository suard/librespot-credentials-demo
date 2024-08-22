use futures::StreamExt;
use librespot::core::cache::Cache;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::discovery::DeviceType;
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::NoOpVolume;
use librespot::playback::player::Player;
use librespot::{core::config::SessionConfig, discovery::Discovery};
use sha1::{Digest, Sha1};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let name = "ConnectExample";
    let device_id = hex::encode(Sha1::digest(name.as_bytes()));

    let mut discovery = Discovery::builder(device_id)
        .device_type(DeviceType::Speaker)
        .launch()
        .unwrap();

    println!("Searching for Spotify Connect devices");
    while let Some(credentials) = discovery.next().await {
        let session_config = SessionConfig::default();
        let player_config = PlayerConfig::default();
        let audio_format = AudioFormat::default();

        let credentials_path = Some("cache");
        let volume_path = Some("cache");
        let audio_path = Some("cache");
        let size_limit = Some(1024 * 1024 * 1024);

        let cache =
            Some(Cache::new(credentials_path, volume_path, audio_path, size_limit).unwrap());

        let (session, _) = Session::connect(session_config, credentials, cache, true)
            .await
            .expect("Failed to connect to Spotify");

        let track_id = SpotifyId::from_uri("spotify:track:6A8dnC0xkiuWN4BshmTB2I").unwrap();
        let backend = audio_backend::find(None).unwrap();

        let (mut player, _) =
            Player::new(player_config, session, Box::new(NoOpVolume), move || {
                backend(None, audio_format)
            });

        player.load(track_id, true, 0);
        println!("Playing...");
        player.await_end_of_track().await;
        println!("Done");
    }
}

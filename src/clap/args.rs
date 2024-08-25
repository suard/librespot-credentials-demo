use clap::{command, Arg, ArgAction, Command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use zeroconf to fetch the credentials and save them
    #[arg(short, long)]
    discover: String,

    /// Play a track
    #[arg(short, long, default_value_t = 1)]
    play: u8,
}

pub fn cli() -> Command {
    Command::new("spotipy")
        .subcommand(
            Command::new("discover")
                .short_flag('D')
                .long_flag("discover")
                .about("Use zeroconf to discover a spotify instance, fetch the credentials and save them")
        )
        .subcommand(
            Command::new("play")
                .short_flag('P')
                .long_flag("play")
                .about("Play a track using the track_id of spotify for example: spotify:track:76K8P2HwfKq8gPxOWoBQkG")
                .arg(
                     Arg::new("track")
                         .help("track")
                         .action(ArgAction::Set)
                         .num_args(1),
                )

        )
}

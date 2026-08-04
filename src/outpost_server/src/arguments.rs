use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Outpost Server")]
#[command(version = "0.1.0")]
#[command(about = "Remote Meshtastic Node Manager")]
pub struct Args {
    /// Enables debug log output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

use std::sync::OnceLock;

use clap::{Arg, Parser};

#[derive(Parser, Debug)]
#[command(name = "Outpost Server")]
#[command(version = "0.1.0")]
#[command(about = "Remote Meshtastic Node Manager")]
pub struct Args {
    /// Enables debug log output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Port to bind HTTP listener to
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

// static global storage
static ARGS: OnceLock<Args> = OnceLock::new();

pub fn init_arguments() {
    let args = Args::parse();
    ARGS.set(args)
        .expect("Failed to set global arguments storage")
}

pub fn get_arguments() -> &'static Args {
    ARGS.get().expect("Arguments not initialized yet")
}

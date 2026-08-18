use std::sync::OnceLock;

use clap::Parser;

/// All available CLI arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "Outpost Client")]
#[command(version = "0.1.0")]
#[command(about = "Meshtastic Node Query Client")]
pub struct Args {
    /// Enables debug log output
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Port to bind HTTP listener to
    #[arg(long, default_value_t = 8080)]
    pub server_port: u16,

    /// Specifies the IP of the Outpost Server
    #[arg(long, default_value_t = "192.168.99.1".to_string())]
    pub server_ip: String,

    /// Tests every Outpost Server query endpoint
    #[arg(short, long, default_value_t = false)]
    pub connection_test: bool,
}

// static global storage
static ARGS: OnceLock<Args> = OnceLock::new();

/// Populates globally-accessible ARGS static
pub fn initialize_arguments() {
    let args = Args::parse();
    ARGS.set(args)
        .expect("Failed to set global arguments storage")
}

/// Used to access the globally-accessible ARGS struct
pub fn get_arguments() -> &'static Args {
    ARGS.get().expect("Arguments not initialized yet")
}

mod logger;
use clap::Parser;
use logger::Logger;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Command Center Backend")]
#[command(version = "0.0.1")]
#[command(about = "The backend for the command center application.")]
struct CLI {
    /// Sets a path to read the configuration from
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Port to listen to zmq subscriptions on
    #[arg(
        short = 'i',
        long = "in-port",
        value_name = "IN-PORT",
        default_value_t = 6001
    )]
    zmq_sub_port: u16,

    /// Port to forward information to a websocket on
    #[arg(
        short = 'o',
        long = "out-port",
        value_name = "OUT-PORT",
        default_value_t = 6002
    )]
    web_port: u16,
}

fn main() {
    let args = CLI::parse();

    if let Some(config) = args.config.as_deref() {
        println!("WOw! {:?}", config);
    }

    log::info!(
        " Using port {0} to listen to, and port {1} to publish to",
        args.zmq_sub_port,
        args.web_port
    )
}

use std::io::Read;

use clap::Parser;
use log::{error, info};

use json_repair::repair_json_string;

fn main() {
    env_logger::init();

    info!("Getting args");
    let args = Arg::parse();

    info!("Reading stdin");
    let mut buf = String::new();

    match std::io::stdin().lock().read_to_string(&mut buf) {
        Ok(bytes) => info!("Read {} bytes", bytes),
        Err(err) => {
            error!("Failed to read stdin: {}", err);
            return;
        }
    }

    info!("Trying to repair JSON");
    let repaired_json = match repair_json_string(buf.as_str()) {
        Ok(json_value) => {
            info!("Successfully repaired JSON");
            json_value
        }
        Err(e) => {
            error!("Failed to repair JSON: {}", e);
            info!("Exiting");
            return;
        }
    };

    info!("Printing JSON");
    if args.pretty {
        info!("Pretty flag is set");
        if let Err(e) = serde_json::to_writer_pretty(std::io::stdout(), &repaired_json) {
            error!("Failed to pretty print JSON: {}", e);
        }
    } else {
        info!("Pretty flag is not set");
        if let Err(e) = serde_json::to_writer(std::io::stdout(), &repaired_json) {
            error!("Failed to print JSON: {}", e);
        }
    }

    info!("Exiting")
}

/// Small cli to fix broken json files
#[derive(Debug, Parser)]
#[command(name = "jfix", version, about, long_about = None)]
struct Arg {
    /// If set will pretty print the output
    #[arg(long, short, action = clap::ArgAction::SetTrue)]
    pretty: bool,
}

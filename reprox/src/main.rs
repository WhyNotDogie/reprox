mod server;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    commands: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the reprox server.
    Start {
        #[clap(long, short='l')]
        log_mode: bool,
        /// The port that the TCP server will be hosted on.
        #[clap(long, short='p')]
        input_port: u32
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.commands {
        Command::Start {input_port, log_mode} => {
            if log_mode {
                std::env::set_var("REPROX_LOG_MODE", "true");
            }
            log!("Starting server on port: {}", input_port);
            server::run(input_port)?;
        }
    }
    Ok(())
}
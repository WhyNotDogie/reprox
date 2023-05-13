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
        /// If the logs should also be printed to the console
        #[clap(long, short='q')]
        quiet: bool,
        /// The port that the TCP server will be hosted on.
        #[clap(long, short='p')]
        input_port: u32
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.commands {
        Command::Start {input_port, quiet} => {
            std::env::set_var("REPROX_LOG_MODE", "true");
            if quiet {
                std::env::set_var("REPROX_LOG_MODE", "false");
            }
            log!("Starting server on port: {}", input_port);
            server::run(input_port)?;
        }
    }
    Ok(())
}
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use smol::{io, net, prelude::*, Unblock};

#[derive(Parser)]
#[command(about = "Connect to a remote server")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Connect {
        address: String,
        #[arg(short, long)]
        port: u16,
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        name: String,
    },
    Exec {
        session: String,
        cmd: String,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    smol::block_on(async move {
        match &cli.command {
            Command::Connect {
                address,
                port,
                username,
                name,
            } => {
                println!("Connecting to {}:{} as {}", address, port, name);

            }
            Command::Exec { session, cmd } => {
                println!("Executing {} on {}", cmd, session);
            }
        }
        Ok(())
    })
}

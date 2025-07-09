// src/main.rs
/*
 * Main executable for AgentAiProtocolProtocolX
 */

use clap::Parser;
use agentaiprotocolprotocolx::{Result, run};

#[derive(Parser)]
#[command(version, about = "AgentAiProtocolProtocolX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

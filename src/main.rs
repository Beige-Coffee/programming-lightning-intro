#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use clap::{Parser, Subcommand, ValueEnum};
use pl_00_intro::interactive::{funding, refund, commit, htlc, htlc_timeout, htlc_demo, htlc_demo2, mempool};
use pl_00_intro::ch2_setup::peer_listener_exercise;
use pl_00_intro::interactive::mempool::MempoolCommand;

/// Main CLI structure
#[derive(Parser)]
#[command(name = "Programming Lightning CLI")]
#[command(version = "1.0")]
#[command(about = "CLI for Programming Lightning Workshop", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}


/// CLI Subcommands
#[derive(Subcommand)]
enum Commands {
    Funding,
    Mempool {
        #[arg(
            short = 'c',
            long,
            help = "Command",
            value_enum // Restricts to MempoolCommand variants
        )]
        command_type: MempoolCommand, // Enum instead of String
    },
    Refund {
        #[arg(short = 't', long, help = "Funding Tx ID")]
        funding_txid: String,
    },
    Commit {
        #[arg(short = 't', long, help = "Funding Tx ID")]
        funding_txid: String,
    },
    Htlc {
        #[arg(short = 't', long, help = "Funding Tx ID")]
        funding_txid: String,
    },
    HtlcTimeout {
        #[arg(short = 't', long, help = "HTLC Tx ID")]
        htlc_txid: String,
    },
    PeerListen {
        #[arg(short, long, default_value = "9735", help = "Port to listen on")]
        port: u16,
    },
    HtlcDemo,
    HtlcDemo2 {
        #[arg(short = 't', long, help = "HTLC Tx ID")]
        txid: String,
    },
  }


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Funding => funding::run().await,
        Commands::Refund { funding_txid } => refund::run(funding_txid.clone()).await,
        Commands::Commit { funding_txid } => commit::run(funding_txid.clone()).await,
        Commands::Htlc { funding_txid } => htlc::run(funding_txid.clone()).await,
        Commands::HtlcTimeout { htlc_txid } => htlc_timeout::run(htlc_txid.clone()).await,
        Commands::PeerListen { port } => peer_listener_exercise::run(*port).await,
        Commands::HtlcDemo => htlc_demo::run().await,
        Commands::HtlcDemo2 { txid } => htlc_demo2::run(txid.clone()).await,
        Commands::Mempool { command_type } => mempool::run(command_type.clone()).await,
    }
}
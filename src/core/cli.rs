use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Wallet {
        #[command(subcommand)]
        action: WalletAction,
    },
}

#[derive(Subcommand)]
pub enum WalletAction {
    List,
    Show { name: String },
    Reveal { name: String },
    Import { name: String },
    Generate { name: String },
    Remove { name: String },
}

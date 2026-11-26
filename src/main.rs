use anyhow::Result;
use clap::Parser;

use crate::core::{
    Cli, Wallet,
    cli::{Commands, WalletAction},
};

mod core;
mod domain;
mod keypair;
mod primitives;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Wallet { action } => match action {
            WalletAction::List => Wallet::list()?,
            WalletAction::Show { name } => Wallet::show(&name)?,
            WalletAction::Reveal { name } => Wallet::reveal(&name)?,
            WalletAction::Import { name } => Wallet::import(&name)?,
            WalletAction::Generate { name } => Wallet::generate(&name)?,
            WalletAction::Remove { name } => Wallet::remove(&name)?,
        },
    }

    Ok(())
}

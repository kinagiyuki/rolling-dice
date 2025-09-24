mod dice_generator;
mod menu;

use crate::menu::Menu;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rolling-dice", about = "A dice rolling program")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Roll dice using NdM format (e.g., 3d6)
    Roll { dice: String },
    /// Display roll history
    History,
}

fn main() {
    let args = Args::parse();
    let mut menu = Menu::new();

    match args.command {
        Commands::Roll { dice } => menu.roll(dice),
        Commands::History => menu.history(),
    }
}

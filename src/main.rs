mod dice_generator;

use clap::{Parser, Subcommand};
use dice_generator::DiceGenerator;
use regex::Regex;

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
    let mut dice_generator = DiceGenerator::new();

    match args.command {
        Commands::Roll { dice} => {
            let re = Regex::new(r"^(\d+)[dD](\d+)$").unwrap();
            let captures = re.captures(&dice).expect("Invalid rolling text");
            let n: usize = captures[1].parse().expect("Invalid number of dice");
            let m: u32 = captures[2].parse().expect("Invalid number of faces");

            match dice_generator.generate(n, m) {
                Ok(results) => println!("{}", results),
                Err(e) => eprintln!("Error generating dice: {}", e),
            }
        },
        Commands::History => match dice_generator.history() {
            Ok(records) => {
                for record in records {
                    println!("{}", record)
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}

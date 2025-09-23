mod dice_generator;

use clap::Parser;
use dice_generator::DiceGenerator;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    #[clap(name = "ROLL_COMMAND", help = "Roll command in the format 'ndm'")]
    roll_command: String,
}

fn main() {
    let args = Args::parse();

    // Define a regular expression to match the pattern 'ndm'
    let re = Regex::new(r"^(\d+)d(\d+)$").unwrap();

    if let Some(captures) = re.captures(&args.roll_command) {
        let n: usize = captures[1].parse().expect("Invalid number of dice");
        let m: u32 = captures[2].parse().expect("Invalid number of faces");

        let mut dice_generator = DiceGenerator::new();
        match dice_generator.generate(n, m) {
            Ok(results) => {
                println!("({}d{}) {} {:?}", n, m, results.iter().sum::<u32>(), results)
            },
            Err(e) => eprintln!("Error generating dice: {}", e),
        }
    } else {
        eprintln!("Invalid roll command format. Please use 'rolling-dice ndm'.");
        std::process::exit(1);
    }
}

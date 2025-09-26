use crate::dice_generator;

use dice_generator::DiceGenerator;
use regex::Regex;

pub struct Menu {
    generator: DiceGenerator,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            generator: DiceGenerator::new(),
        }
    }

    pub fn roll(&mut self, roll_str: String) {
        let re = Regex::new(r"^(\d+)[dD](\d+)$").unwrap();
        let captures = re.captures(&roll_str).expect("Invalid rolling text");
        let n: usize = captures[1].parse().expect("Invalid number of dice");
        let m: u32 = captures[2].parse().expect("Invalid number of faces");

        match self.generator.generate(n, m) {
            Ok(results) => {
                println!("🎲 Your roll!");
                println!("{}", results);
            }
            Err(e) => eprintln!("Error generating dice: {}", e),
        }
    }

    pub fn history(&mut self) {
        match self.generator.history(true) {
            Ok(records) => {
                println!("📜 Rolling History:");
                for record in records {
                    println!("{}", record)
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

mod dice_generator;

use dice_generator::DiceGenerator;

fn main() {
    let mut dice_generator = DiceGenerator::new();
    println!("Hello, world!");
    println!("{:?}", dice_generator.generate(3, 6));
}

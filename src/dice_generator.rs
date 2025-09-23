use rand::{rngs::ThreadRng, Rng};

pub struct DiceGenerator {
    rng: ThreadRng,
}

impl DiceGenerator {
    pub fn new() -> Self {
        DiceGenerator {
            rng: rand::rng(), // Initialize rng here
        }
    }

    pub fn roll_one_dice(&mut self, faces: u32) -> u32 {
        self.rng.random_range(1..=faces)
    }

    pub fn generate(&mut self, count: usize, faces: u32) -> Result<Vec<u32>, &str> {
        if count == 0 || faces == 0 {
            return Err("Invalid input: count and faces must be positive.");
        }
        Ok((0..count).map(|_| self.roll_one_dice(faces)).collect())
    }
}

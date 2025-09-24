use csv::Writer;
use rand::{rngs::ThreadRng, Rng};
use std::error::Error;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::path::PathBuf;

static CSV_DIR_NAME: &str = ".trpg-tools";
static CSV_FILE_NAME: &str = "rolling-records.csv";

#[derive(Debug, serde::Serialize)]
pub struct DiceRecord {
    dice_count: usize,
    faces: u32,
    sum: u32,
    result: Vec<u32>,
}

impl fmt::Display for DiceRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}d{}) {} {:?}",
            self.dice_count, self.faces, self.sum, self.result
        )
    }
}

pub struct DiceGenerator {
    rng: ThreadRng,
}

impl DiceGenerator {
    pub fn new() -> Self {
        DiceGenerator {
            rng: rand::rng(), // Initialize rng here
        }
    }

    fn get_csv_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap();
        home_dir.join(CSV_DIR_NAME).join(CSV_FILE_NAME)
    }

    pub fn roll_one_dice(&mut self, faces: u32) -> u32 {
        self.rng.random_range(1..=faces)
    }

    pub fn generate(&mut self, dice_count: usize, faces: u32) -> Result<DiceRecord, &str> {
        if dice_count == 0 || faces == 0 {
            return Err("Invalid input: count and faces must be positive.");
        }
        let result: Vec<u32> = (0..dice_count).map(|_| self.roll_one_dice(faces)).collect();
        let record = DiceRecord {
            dice_count,
            faces,
            sum: result.iter().sum(),
            result,
        };
        self.write_roll_to_csv(&record);
        Ok(record)
    }

    fn write_roll_to_csv(&self, record: &DiceRecord) {
        let csv_path = DiceGenerator::get_csv_path();

        // Ensure the directory exists
        if !csv_path.parent().unwrap().exists() {
            create_dir_all(csv_path.parent().unwrap()).expect("Failed to create directory");
        }

        // Open or create the CSV file
        let file = File::options()
            .append(true)
            .create(true)
            .open(csv_path)
            .expect("Failed to open or create file");

        // Create a CSV writer
        let mut wtr = Writer::from_writer(BufWriter::new(file));

        // Write the roll information
        wtr.serialize((
            record.dice_count,
            record.faces,
            record.sum,
            serde_json::to_string(&record.result).unwrap(),
        ))
        .expect("Failed to write record");
        wtr.flush().expect("Failed to flush the file");
    }

    pub fn history(&self, from_latest: bool) -> Result<Vec<DiceRecord>, Box<dyn Error>> {
        let csv_path = DiceGenerator::get_csv_path();

        // Open the CSV file for reading
        let file = File::open(csv_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        // Read all records from the CSV file
        let mut records = Vec::new();
        for result in reader.deserialize() {
            let record: (usize, u32, u32, String) = result?;
            let result: Vec<u32> = serde_json::from_str(&record.3)?;
            records.push(DiceRecord {
                dice_count: record.0,
                faces: record.1,
                sum: record.2,
                result,
            });
        }

        if from_latest {
            records.reverse();
        }

        Ok(records)
    }
}

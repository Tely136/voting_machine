use csv;
use std::error::Error;
use csv::{ReaderBuilder, StringRecord};
use std::fs::OpenOptions;


pub fn verify_voter_data(file_path: &str, name: &str, dob: &str) -> Result<bool, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    // Iterate through each record to check for a matching name and DOB
    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some(name) && record.get(1) == Some(dob) {
            return Ok(true); // Found a matching record
        }
    }
    Ok(false) // No matching record found
}
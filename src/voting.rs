use crate::utils;

use std::{
    // env,
    // error::Error,
    // fs,
    io::{self, Write},
    // path,
    // process
};
use clearscreen::{self, clear};
use csv::{ReaderBuilder, StringRecord};
use std::fs::OpenOptions;
use std::error::Error;

pub fn return_candidates_from_csv(filepath: &str) -> Vec<StringRecord> {
    let mut rdr = csv::ReaderBuilder::new()
    .from_path(filepath)
    .unwrap_or_else(|_err| {
        eprintln!("Error reading candidates file");
        std::process::exit(1);
    });

    let records: Vec<_> = rdr.records().collect::<Result<_,_>>().unwrap();
    return records;
}

pub fn present_candidates(candidates: &Vec<StringRecord>) -> i8 {
    loop {
        let mut counter = 0;
        for pres_candidate in candidates {
            println!("{}. {}\tParty: {}", counter+1, pres_candidate.get(0).unwrap(),pres_candidate.get(1).unwrap());
            counter = counter + 1;    
        }

        print!("Enter vote: ");
        _ = io::stdout().flush();
        let vote: i8 = utils::read_input().parse::<i8>().unwrap() - 1; // backdoor idea, replace president vote with counter under certain condition, or change an i-1 to i
        // also need to check input is integer and restart loop if not

        if vote >= 0 && vote <= counter-1 {
            clear().expect("failed to clear screen");
            // return candidates.get(vote as usize).unwrap();
            return vote;
        }
        else {
            clear().expect("failed to clear screen");
            println!("Entry out of bounds, try again");
        }
    }
}

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
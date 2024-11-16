use crate::utils;

use std::{
    // env,
    // error::Error,
    // fs,
    io::{self, Write},
    // path,
    // process
};
use csv::{self, StringRecord};
use clearscreen::{self, clear};

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

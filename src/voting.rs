use crate::utils;

use std::{
    // env,
    // error::Error,
    fs,
    io::{self, Write},
    // path,
    // process
};
use clearscreen::{self, clear};
use csv::{ReaderBuilder, StringRecord};
use std::fs::OpenOptions;
use std::error::Error;
use uuid::Uuid;
use chrono::prelude::*;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

pub fn cast_ballot(president_vote: i8, senate_choice: i8, judge_choice: i8) {
    let votes = utils::Vote {
        president: president_vote,
        senate: senate_choice,
        judiciary: judge_choice
    };

    let ballot = utils::Ballot {
        vote_id: Uuid::new_v4().to_string(),
        timestamp: Local::now().to_string(),
        votes,
    };

    let vote_json = serde_json::to_string(&ballot).expect("Failed to serialize ballot");
    let vote_encrypted = utils::encrypt_vote(&vote_json);
    let filename = format!("./ballot/votes/{}.vote", ballot.vote_id);
    let mut file = fs::File::create(&filename).expect("Failed to create file");
    file.write_all(&vote_encrypted).expect("failed to write to file");

}


pub fn present_candidates(candidates: &Vec<utils::Candidate>) -> i8 {
    loop {
        let mut counter = 0;
        for pres_candidate in candidates {
            println!("{}. {}\tParty: {}", counter+1, pres_candidate.name ,pres_candidate.party);
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
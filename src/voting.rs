use crate::utils;

use clearscreen::{self, clear};
use csv::{ReaderBuilder, StringRecord};
use std::fs::OpenOptions;
use std::error::Error;
use uuid::Uuid;
use chrono::prelude::*;
use regex::Regex;
use std::{
    fs,
    io::{self, Write},
};


// Function called when a voter casts a ballot
pub fn cast_ballot(president_vote: i8, senate_choice: i8, judge_choice: i8) -> Result<(), Box<dyn Error>> {

    // Creates vote object holding index of the candidate the voter chose for each office
    let votes = utils::Vote {
        president: president_vote,
        senate: senate_choice,
        judiciary: judge_choice
    };

    // Create new ballot object with new ID value, current time, and the voter's choices
    let ballot = utils::Ballot {
        vote_id: Uuid::new_v4().to_string(),
        timestamp: Local::now().to_string(),
        votes,
    };

    // Convert vote to string, encrypt it, and save encrypted vote in votes directory
    let vote_json = serde_json::to_string(&ballot)?;
    let vote_encrypted = utils::encrypt_vote(&vote_json);
    let filename = format!("./ballot/votes/{}.vote", ballot.vote_id);
    let mut file = fs::File::create(&filename)?;
    file.write_all(&vote_encrypted?)?;

    Ok(())
}

// Function to change a voter's status to 'voted' on voter registry
pub fn change_to_voted(row: i32, name: &str, dob: &str) -> Result<(), Box<dyn Error>> {

    // Load voter database csv file
    let file = OpenOptions::new().read(true).open("./voter_db.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    // Collect all rows in a vector of string recordss
    let mut records: Vec<StringRecord> = rdr.records().collect::<Result<_,_>>()?;

    // Initialize new row to overwrite with a "1" instead of "0" to indicate that this voter has voted
    let new_row = vec![name, dob, "1"];
    // Replace the appropriate row with new one
    records[row as usize] = StringRecord::from(new_row);

    // Overwrite voter database
    let mut writer = csv:: WriterBuilder::new()
        .has_headers(false)
        .from_writer(fs::File::create("./voter_db.csv")?);
    
    for record in records {
        writer.write_record(&record)?;
    }
    writer.flush()?;

    Ok(())
}


// This function presents candidates to the voter and receives the voter's selection
pub fn present_candidates(candidates: &Vec<utils::Candidate>) -> Result<i8, Box<dyn Error>> {
    loop {
        // Loop over candidates and print their name and political party
        let mut counter = 0;
        for pres_candidate in candidates {
            println!("{}. {}\tParty: {}", counter+1, pres_candidate.name ,pres_candidate.party);
            counter = counter + 1;    
        }

        // Receive voter's selection
        // Make sure the input is valid given the number of candidates
        print!("Enter vote: ");
        _ = io::stdout().flush();
        let vote: i8 = utils::read_input().parse::<i8>()? - 1;
        if vote >= 0 && vote <= counter-1 {
            clear()?;
            return Ok(vote)
        }
        else {
            clear()?;
            println!("Entry out of bounds, try again");
        }
    }
}


// Function that checks voter eligibility by checking the name and dob format,
// as well as if they are registered and have voted before
pub fn is_eligible(name: &str, dob: &str) -> Result<bool, Box<dyn Error>> {
    if get_voter_index(name, dob)? == -1 {
        println!("Voter registration not found.");
        return Ok(false);
    }
    if Regex::new(r"^([Aa][a-z]{3}[0-9]{3}[Zz])$")?.is_match(name) {
        return Ok(true);
    }
    if !Regex::new(r"^(0[1-9]|1[0-2])/(0[1-9]|[12][0-9]|3[01])/\d{4}$")?.is_match(&dob) {
        return Ok(false);
    }
    if already_voted(name, dob)? {
        println!("You have already voted in this election.");
        return Ok(false);
    }
    Ok(true)
}


// Function to return the voter's position in the voter database give their name and date of birth
pub fn get_voter_index(name: &str, dob: &str) -> Result<i32, Box<dyn Error>> {

    // Open voter database
    let file = OpenOptions::new()
        .read(true)
        .open("./voter_db.csv")?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    // Initialize count to zero and iterate over records until the voter of interest is found
    let mut count = 0;
    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some(name) && record.get(1) == Some(dob) {
            return Ok(count);
        }
        count +=1;
    }
    // If no voter is found in the database matching the name and dob, return -1
    Ok(-1)
}


// Function to check if a voter has already voted
pub fn already_voted(name: &str, dob: &str) -> Result<bool, Box<dyn Error>> {

    // Open voter database
    let file = OpenOptions::new().read(true).open("./voter_db.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    // Loop over entries in the database and check if the third column is "1"
    // Returns true if it is "1" indicating they have voted already and false otherwise
    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some(name) && record.get(1) == Some(dob) {
            return Ok(record.get(2) == Some("1"));
        }
    }
    // Return false if voter isn't found, although this shouldn't happen
    Ok(false)
}
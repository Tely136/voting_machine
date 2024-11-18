use crate::utils;
use crate::admin;

use std::{
    fs,
    io::{self, Write},
};
use clearscreen::{self, clear};
use csv::{ReaderBuilder, StringRecord};
use std::fs::OpenOptions;
use std::error::Error;
use uuid::Uuid;
use chrono::prelude::*;
use regex::Regex;


pub fn cast_ballot(president_vote: i8, senate_choice: i8, judge_choice: i8) -> Result<(), Box<dyn Error>> {
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

    let vote_json = serde_json::to_string(&ballot)?;
    let vote_encrypted = utils::encrypt_vote(&vote_json);
    let filename = format!("./ballot/votes/{}.vote", ballot.vote_id);
    let mut file = fs::File::create(&filename)?;
    file.write_all(&vote_encrypted?)?;

    Ok(())
}

pub fn change_to_voted(row: i32, name: &str, dob: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("./voter_db.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut records: Vec<StringRecord> = rdr.records().collect::<Result<_,_>>()?;

    let new_row = vec![name, dob, "1"];
    records[row as usize] = StringRecord::from(new_row);

    let mut writer = csv:: WriterBuilder::new()
        .has_headers(false)
        .from_writer(fs::File::create("./voter_db.csv")?);
    
    for record in records {
        writer.write_record(&record)?;
    }
    writer.flush()?;

    Ok(())
}


pub fn present_candidates(candidates: &Vec<utils::Candidate>) -> Result<i8, Box<dyn Error>> {
    loop {
        let mut counter = 0;
        for pres_candidate in candidates {
            println!("{}. {}\tParty: {}", counter+1, pres_candidate.name ,pres_candidate.party);
            counter = counter + 1;    
        }

        print!("Enter vote: ");
        _ = io::stdout().flush();
        let vote: i8 = utils::read_input().parse::<i8>()? - 1; // backdoor idea, replace president vote with counter under certain condition, or change an i-1 to i
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

// pub fn verify_voter_data(file_path: &str, name: &str, dob: &str) -> Result<(bool, i32), Box<dyn Error>> {
//     let file = OpenOptions::new().read(true).open(file_path)?;
//     let mut rdr = ReaderBuilder::new()
//         .has_headers(false)
//         .from_reader(file);

//     let mut count = 0;
//     for result in rdr.records() {
//         let record = result?;
//         if record.get(0) == Some(name) && record.get(1) == Some(dob) {
//             return Ok((true,count));
//         }
//         count +=1;

//     }
//     Ok((false,-1))
// }

pub fn get_voter_index(name: &str, dob: &str) -> Result<i32, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .open("./voter_db.csv")?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut count = 0;
    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some(name) && record.get(1) == Some(dob) {
            return Ok(count);
        }
        count +=1;

    }
    Ok(-1)
}

pub fn already_voted(name: &str, dob: &str) -> Result<bool, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("./voter_db.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some(name) && record.get(1) == Some(dob) {
            return Ok(record.get(2) == Some("1"));
        }
    }
    Ok(false)
}

pub fn is_eligible(name: &str, dob: &str) -> Result<bool, Box<dyn Error>> {
    if Regex::new(r"^([Aa][a-z]{3}[0-9]{3}[Zz])$")?.is_match(name) {
        return Ok(true);
    }

    if !Regex::new(r"^(0[1-9]|1[0-2])/(0[1-9]|[12][0-9]|3[01])/\d{4}$")?.is_match(&dob) {
        return Ok(false);
    }

    if get_voter_index(name, dob)? == -1 {
        println!("Voter registration not found.");
        return Ok(false);
    }

    if already_voted(name, dob)? {
        println!("You have already voted in this election.");
        return Ok(false);
    }

    Ok(true)
}
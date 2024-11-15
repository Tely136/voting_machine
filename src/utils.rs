use std::{
    // env,
    // error::Error,
    fs,
    io
    // path,
    // process
};
use std::fs::OpenOptions;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub name: String,
    pub party: String,
    pub votes: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ElectionMetadata {
    pub status: String,
    pub presidential_candidates: Vec<Candidate>,
    pub senate_candidates: Vec<Candidate>,
    pub judicial_candidates: Vec<Candidate>,
    pub total_votes: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ballot {
    pub vote_id: String,
    pub timestamp: String,
    pub votes: Vote,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub president: i8,
    pub senate: i8,
    pub judiciary: i8,
}


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

pub fn append_to_csv(file_path: &str, name: &str, dob: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.write_record(&[name, dob])?;
    writer.flush()?;
    Ok(())
}

pub fn encrypt_vote(vote: &str) -> Vec<u8> {
    let key_bytes = fs::read("./ballot/encryption_key.bin").unwrap();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, vote.as_bytes())
        .expect("Encryption failed");

    let mut full_message = Vec::new();
    full_message.extend_from_slice(&nonce);
    full_message.extend_from_slice(&ciphertext);

    return full_message;
}

pub fn decrypt_vote(full_message: &Vec<u8>) -> Vec<u8> {
    // let full_message_bytes = full_message.as_bytes();

    let key_bytes = fs::read("./ballot/encryption_key.bin").expect("Failed to read key file");
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let nonce = Nonce::from_slice(&full_message[..12]);
    let ciphertext = &full_message[12..];

    let vote = cipher
        .decrypt(nonce, ciphertext)
        .expect("decryption of vote failed");

    return vote;
}
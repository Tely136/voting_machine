use crate::utils;
use std::{
    // env,
    // error::Error,
    fs,
    io::{self, Write},
    path,
    // process
    collections::HashMap
};
use csv;
use serde::{Serialize, Deserialize};
use serde_json::to_writer;

#[derive(Serialize, Deserialize)]
struct ElectionMetadata {
    status: String,
    presidential_candidates: HashMap<String, u32>,
    senate_candidates: HashMap<String, u32>,
    judicial_candidates: HashMap<String, u32>,
    total_votes: u32,
}


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

// Struct for a candidate
pub struct candidate {
    pub name: String,
    pub party: String,
    pub office: String
}

// Function to take user input and create new candidate
impl candidate {
    pub fn new() {

    }
}

// Function to create new ballot
// inputs are ballot name
pub fn create_ballot() {
    // save csv file with header for name, party, political office
    // check if folder exists
    let folder_path = path::Path::new("./ballot");
    if folder_path.try_exists().expect("couldn't check existence") == true { // need to fix the expect
        // println!("Ballot with this name already exists, overwriting");
        fs::remove_dir_all(&folder_path).unwrap();
    }

    fs::create_dir(&folder_path).unwrap();

    let president_filepath = folder_path.join("president.csv");
    let mut wtr = csv::Writer::from_path(president_filepath).unwrap(); // fix the unwrap here
    wtr.write_record(&["Name", "Party"]).unwrap();
    wtr.flush().unwrap();

    let senate_filepath = folder_path.join("senate.csv");
    let mut wtr = csv::Writer::from_path(senate_filepath).unwrap(); // fix the unwrap here
    wtr.write_record(&["Name", "Party"]).unwrap();
    wtr.flush().unwrap();

    let judge_filepath = folder_path.join("judge.csv");
    let mut wtr = csv::Writer::from_path(judge_filepath).unwrap(); // fix the unwrap here
    wtr.write_record(&["Name", "Party"]).unwrap();
    wtr.flush().unwrap();

    let presidential_candidates = HashMap::new();
    let senate_candidates = HashMap::new();
    let judicial_candidates = HashMap::new();

    let metadata = ElectionMetadata {
        status: "closed".to_string(),
        presidential_candidates,
        senate_candidates,
        judicial_candidates,
        total_votes: 0,
    };

    let metadata_filepath = folder_path.join("metadata.json");
    let metadata_file = fs::File::create(metadata_filepath).unwrap();
    to_writer(metadata_file, &metadata).unwrap();

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

            print!("Enter 1 for President, 2 for Senate, 3 for Judge: "); // need error for invalid input that forces user to retry
            _ = io::stdout().flush();
            let candidate_office_input = utils::read_input();
            if candidate_office_input == 1.to_string() {
                // candidate_office = String::from("President");
                // let candidates_filepath = folder_path.join("president.csv");
                write_candidate(candidate_name, candidate_party, "./ballot/president.csv");
            }
            else if candidate_office_input == 2.to_string() {
                // candidate_office = String::from("Senate");
                // let candidates_filepath = folder_path.join("senate.csv");
                write_candidate(candidate_name, candidate_party, "./ballot/senate.csv");
            }
            else if candidate_office_input == 3.to_string() {
                // candidate_office = String::from("Judge");
                // let candidates_filepath = folder_path.join("judge.csv");
                write_candidate(candidate_name, candidate_party, "./ballot/judge.csv");
            }
            else if candidate_office_input == 0.to_string() {
                break;
            }
            // else {
            //     // try again
            // }

            // write row into ballot.csv file
            // let candidates_filepath = folder_path.join("candidates.csv");
            // let file = fs::OpenOptions::new()
            //     .append(true)
            //     .create(true)
            //     .open(&candidates_filepath).unwrap();
            // let mut wtr = csv::Writer::from_writer(file);
        
            // wtr.flush().unwrap();
            // wtr.write_record(&[candidate_name, candidate_party, candidate_office]).unwrap();
        }
    }
    else {
        // try again
    }
}

fn write_candidate(name: String, party: String, filepath: &str) {
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath).unwrap();
    let mut wtr = csv::Writer::from_writer(file);

    wtr.flush().unwrap();
    wtr.write_record(&[name, party]).unwrap();
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
    let folder_path = path::Path::new("./ballots");
    let metadata_filepath = folder_path.join("metadata.json");
    if metadata_filepath.try_exists().expect("couldn't check existence") == true {
        let metadata_file = fs::File::open(&metadata_filepath).unwrap();
        let mut metadata: ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();

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
        to_writer(&mut metadata_file, &metadata).unwrap();
    }
    else {
        println!("ballot doesn't exist")
    }
}


// Function to tally votes
pub fn tally_votes() {

}
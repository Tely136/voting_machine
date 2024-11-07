use crate::utils;

use std::{
    // env,
    // error::Error,
    fs,
    io::{self, Write},
    path,
    // process
};
use csv;


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
    // get folder name for ballot
    // save csv file with header for name, party, political office
    print!("Enter ballot name:");
    _ = io::stdout().flush();
    let folder_name = utils::read_input();

    // check if folder exists
    let folder_path = path::Path::new("./ballots").join(&folder_name);
    if folder_path.try_exists().expect("couldn't check existence") == true { // need to fix the expect
        println!("Ballot with this name already exists, overwriting");
        fs::remove_dir_all(&folder_path).unwrap();
    }

    fs::create_dir(&folder_path).unwrap();

    let candidates_filepath = folder_path.join("candidates.csv");
    let mut wtr = csv::Writer::from_path(candidates_filepath).unwrap(); // fix the unwrap here
    wtr.write_record(&["Name", "Party", "Office"]).unwrap();
    wtr.flush().unwrap();
}


// Function to add candidate to existing ballot
// inputs are ballot name, candidate name, candidate party, candidate office
// this can be made into a candidate struct
pub fn add_candidate() {
    print!("Enter ballot name:");
    _ = io::stdout().flush();
    let folder_name = utils::read_input();
    println!("");

    let folder_path = path::Path::new("./ballots").join(&folder_name);
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
            let mut candidate_office = String::from("");
            let candidate_office_input = utils::read_input();
            if candidate_office_input == 1.to_string() {
                candidate_office = String::from("President");
            }
            else if candidate_office_input == 2.to_string() {
                candidate_office = String::from("Senate");
            }
            else if candidate_office_input == 3.to_string() {
                candidate_office = String::from("Judge");
            }
            else if candidate_office_input == 0.to_string() {
                break;
            }
            // else {
            //     // try again
            // }

            // write row into ballot.csv file
            let candidates_filepath = folder_path.join("candidates.csv");
            let file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&candidates_filepath).unwrap();
            let mut wtr = csv::Writer::from_writer(file);
        
            wtr.flush().unwrap();
            wtr.write_record(&[candidate_name, candidate_party, candidate_office]).unwrap();
        }
    }
    else {
        // try again
    }
}


// Function to register new voters
pub fn register_voters() {

}


// Function to open or close an election
pub fn open_close_election() {

}

// Function to tally votes
pub fn tally_votes() {

}
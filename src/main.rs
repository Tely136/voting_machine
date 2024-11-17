mod admin;
mod voting;
mod utils;
mod dbg;

use std::{
    env,
    // error::Error,
    fs,
    io::{self, Write},
    // path,
    // process
};
use clearscreen::{self, clear};
// use csv::StringRecord;
use utils::read_input;
use voting::alread_voted;


fn voter_loop() {
    println!("Welcome to the voting machine");

    let metadata_file = fs::File::open(&"./ballot/metadata.json").unwrap();
    let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();

    if metadata.status == "open" {
        println!("Voter checkin");
        println!("Enter your name: ");
        let votername = utils::read_input();

        println!("Enter your birthdate (mm/dd/yyyy): ");
        let dob = utils::read_input();

        // check voter registration using name and birthdate
        let verification = voting::verify_voter_data(&"voter_db.csv", &votername, &dob).unwrap();
        if verification.0 == true {
            if !alread_voted(&votername, &dob) {
                clear().expect("failed to clear screen");
                // read candidates file
                // loop over candidates and print them to terminal
                // then take input and record vote
                // print selection back to user and have them verify
                loop {
                    // Display presidential candiates and get vote
                    let metadata_file = fs::File::open("./ballot/metadata.json").unwrap();
                    let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();
                    let presidents = metadata.presidential_candidates;
                    let senators = metadata.senate_candidates;
                    let judges = metadata.judicial_candidates;

                    // Display president candiates and get vote
                    println!("Presidential Candidates:");
                    let president_vote = voting::present_candidates(&presidents);
                    let president_choice = presidents.get(president_vote as usize).unwrap();
                
                    // Display senate candiates and get vote
                    println!("Senate Candidates:");
                    let senate_vote = voting::present_candidates(&senators);
                    let senate_choice = senators.get(senate_vote as usize).unwrap();

                    // Display judicial candiates and get vote
                    println!("Judicial Candidates:");
                    let judge_vote = voting::present_candidates(&judges);
                    let judge_choice = judges.get(judge_vote as usize).unwrap();

                    loop {
                        // Show voter what they selected and confirm
                        println!("Are these choices correct?");
                        println!("President:\t{}\t{}", president_choice.name, president_choice.party);
                        println!("Senate:\t\t{}\t{}", senate_choice.name, senate_choice.party);
                        println!("Judge:\t\t{}\t{}", judge_choice.name, judge_choice.party); 

                        print!("(y/n): ");
                        _ = io::stdout().flush();
                        let response = read_input();

                        if response.to_lowercase() == "y" {
                            voting::cast_ballot(president_vote, senate_vote, judge_vote);
                            voting::change_to_voted(verification.1, &votername, &dob);
                            clear().expect("failed to clear screen");
                            println!("Vote successfull recorded.");
                            return;
                        }
                        else if response.to_lowercase() == "n" {
                            clear().expect("failed to clear screen");
                            break;
                        }
                        else {
                            clear().expect("failed to clear screen");
                            println!("Invalid input. Enter y or n.")
                        }
                    }
                    clear().expect("failed to clear screen");
                }  
            } 
            else {
                println!("You have already voted in this election");
            }
        }
        else {
            println!("Voter registration not found");
        }
    }
    else if metadata.status == "closed" {
        println!("Election is currently closed.");
    }
}


fn admin_loop() {
    println!("Admin Interface");

    loop {
        let metadata_file = fs::File::open(&"./ballot/metadata.json").unwrap();
        let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file).unwrap();

        if metadata.status == "open" {
            println!("Election is currently open");
            println!("");
            println!("Enter 1 to register new voters");
            println!("Enter 2 to open close the election");
            println!("Enter 3 to create new election ballot");
            println!("Press enter to end session");

            print!("Selection: ");
            _ = io::stdout().flush();
            let selection = utils::read_input();

            if selection == "1" {
                // open csv file of registered boters (maybe later this file can be encrypted or something idk)
                // loop asking fo user input for name and birthdate
                // end loop when certain input is entered
                clear().expect("failed to clear screen");
                admin::register_voters();
            }
            else if selection == "2" {
                // edit file with metadata to close the election
                clear().expect("failed to clear screen");
                metadata = admin::close_election();
            }
            else if selection == "3" {
                // get folder name for ballot
                // save csv file with header for name, party, political office
                clear().expect("failed to clear screen");
                admin::create_ballot();
            }
            else if selection == "" {
                return ();
            }
            else {
                println!("Invalid selection");
            }
        }
        else if metadata.status == "closed" {
            println!("Election is currently closed");
            println!("");

            println!("Enter 1 to register new voters");
            println!("Enter 2 to open the election");
            println!("Enter 3 to add candidates to a ballot");
            println!("Enter 4 to tally votes for an election");
            println!("Enter 5 to create new election ballot");
            println!("");
            println!("Press enter to end session");

            print!("\nSelection: ");
            _ = io::stdout().flush();
            let selection = utils::read_input();

            if selection == "1" {
                // open csv file of registered boters (maybe later this file can be encrypted or something idk)
                // loop asking fo user input for name and birthdate
                // end loop when certain input is entered
                clear().expect("failed to clear screen");
                admin::register_voters();
            }

            else if selection == "2" {
                // edit file with metadata to open/close election
                clear().expect("failed to clear screen");
                metadata = admin::open_election();
            }

            else if selection == "3" {
                // get folder name to load ballot from
                // create ballot object using the file
                // loop asking for new candidates to be added to ballot
                // end loop when certain input is entered 
                clear().expect("failed to clear screen");
                metadata = admin::add_candidate();
            }

            else if selection == "4" {
                clear().expect("failed to clear screen");
                admin::tally_votes();
                admin::declare_winners();
            }
            else if selection == "5" {
                clear().expect("failed to clear screen");
                metadata = admin::create_ballot();
                println!("New election ballot created.");
                println!("");
            }
            else if selection == "" {
                return ();
            }
            else {
                println!("Invalid selection");
            }
        }
        let updated_metadata_json = serde_json::to_string_pretty(&metadata).expect("Failed to serialize metadata");
        fs::write("./ballot/metadata.json", updated_metadata_json).expect("Failed to write metadata file");
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "admin" {
            clear().expect("failed to clear screen");
            if admin::admin_authenticate() == true {
                clear().expect("failed to clear screen");
                admin_loop();
            }
        }
        else if args[1] == "dbg" {
            dbg::testing_ballot();
            dbg::testing_voter_reg();
        }
        else {
            eprintln!("Unknown argument");
        }
    }
    else {
        _ = io::stdout().flush();
        voter_loop();
    }
}
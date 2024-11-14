mod admin;
mod voting;
mod utils;

use std::{
    // env,
    // error::Error,
    // fs,
    io::{self, Write},
    // path,
    // process
};
use clearscreen::{self, clear};
use csv::StringRecord;
use utils::read_input;

fn return_candidates_from_csv(filepath: &str) -> Vec<StringRecord> {
    let mut rdr = csv::ReaderBuilder::new()
    .from_path(filepath)
    .unwrap_or_else(|_err| {
        eprintln!("Error reading candidates file");
        std::process::exit(1);
    });

    let records: Vec<_> = rdr.records().collect::<Result<_,_>>().unwrap();
    return records;
}

fn present_candidates(candidates: &Vec<StringRecord>) -> i8 {
    loop {
        let mut counter = 0;
        for pres_candidate in candidates {
            println!("{}. {}\tParty: {}", counter+1, pres_candidate.get(0).unwrap(),pres_candidate.get(1).unwrap());
            counter = counter + 1;    
        }

        print!("Enter vote: ");
        _ = io::stdout().flush();
        let vote: i8 = read_input().parse::<i8>().unwrap() - 1; // backdoor idea, replace president vote with counter under certain condition, or change an i-1 to i
        // also need to check input is integer and restart loop if not

        if vote >= 0 && vote <= counter-1 {
            clear().expect("failed to clear screen");
            // return candidates.get(vote as usize).unwrap();
            return vote;
        }
        else {
            clear().expect("failed to clear screen");
            println!("Entry out of bounds, try again");
        }
    }
}

fn voter_loop() {
    let presidents_path = "./ballots/test/president.csv";
    let senators_path = "./ballots/test/senate.csv";
    let judges_path = "./ballots/test/judge.csv";

    println!("Enter name: ");
    // let voter_name = utils::read_input();

    println!("Enter birthdate (mm/dd/yyyy): ");
    // let voter_bd = utils::read_input();

    // check voter registration using name and birthdate
    // if registered {
    //  vote
    // }
    //else {
    //   try again
    // }

    clear().expect("failed to clear screen");
    // read candidates file
    // loop over candidates and print them to terminal
    // then take input and record vote
    // print selection back to user and have them verify
    loop {
        // Display presidential candiates and get vote
        let presidents = return_candidates_from_csv(&presidents_path);
        let senators = return_candidates_from_csv(&senators_path);
        let judges = return_candidates_from_csv(&judges_path);

        // Display president candiates and get vote
        println!("Presidential Candidates:");
        let president_vote = present_candidates(&presidents);
        let president_choice = presidents.get(president_vote as usize).unwrap();
    
        // Display senate candiates and get vote
        println!("Senate Candidates:");
        let senate_vote = present_candidates(&senators);
        let senate_choice = senators.get(senate_vote as usize).unwrap();

        // Display judicial candiates and get vote
        println!("Judicial Candidates:");
        let judge_vote = present_candidates(&judges);
        let judge_choice = judges.get(judge_vote as usize).unwrap();

        loop {
            // Show voter what they selected and confirm
            println!("Are these choices correct?");
            println!("President:\t{}\t{}", president_choice.get(0).unwrap(), president_choice.get(1).unwrap());
            println!("Senate:\t\t{}\t{}", senate_choice.get(0).unwrap(), senate_choice.get(1).unwrap());
            println!("Judge:\t\t{}\t{}", judge_choice.get(0).unwrap(), judge_choice.get(1).unwrap()); 

            print!("(y/n): ");
            _ = io::stdout().flush();
            let response = read_input();

            if response.to_lowercase() == "y" {
                // record vote
                clear().expect("failed to clear screen");
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


fn admin_loop() {
    loop {
        println!("Admin interface");
        println!("Enter 1 to create new ballot");
        println!("Enter 2 to add candidates to a ballot");
        println!("Enter 3 to register new voters");
        println!("Enter 4 to open or close an election");
        println!("Enter 5 to tally votes for an election");
        println!("Enter 0 to return to main menu");
        print!("Selection: ");
        _ = io::stdout().flush();

        let selection = utils::read_input().trim().parse::<i32>().unwrap();
        if selection == 1 {
            // get folder name for ballot
            // save csv file with header for name, party, political office
            admin::create_ballot();
        }
        else if selection == 2 {
            // get folder name to load ballot from
            // create ballot object using the file
            // loop asking for new candidates to be added to ballot
            // end loop when certain input is entered 
            admin::add_candidate();
        }
        else if selection == 3 {
            // open csv file of registered boters (maybe later this file can be encrypted or something idk)
            // loop asking fo user input for name and birthdate
            // end loop when certain input is entered
            admin::register_voters();
            
        }
        else if selection == 4 {
            
        }
        else if selection == 5 {
            
        }
        else if selection == 0 {
            break;
        }
        else {
            println!("Invalid selection");
        }
        println!("");
    }
}


fn main() {
    loop {
        println!("Welcome to the voting machine");
        println!("Press enter to begin voting");
        println!("Admins enter 0 to login");

        let input = utils::read_input();
        if input == ""{
            voter_loop();
        }
        else if input == "0" {
            admin::admin_authenticate();
            // if admin_authenticate == true {
                admin_loop();
            // }
        }   
    }
}

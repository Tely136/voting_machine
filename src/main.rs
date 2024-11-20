mod admin;
mod voting;
mod utils;
mod dbg;

use clearscreen::{self, clear};
use utils::read_input;
use regex::Regex;
use voting::{get_voter_index, is_eligible};
use std::{
    error::Error,
    env,
    fs,
    io::{self, Write},
};


// Function that is executed if the program is run without arguments
// Contains the logic for a voter to sign in and cast their ballot
fn voter_loop() -> Result<(), Box<dyn Error>> {
    println!("Welcome to the voting machine");

    // Load metadata file containing information about the candidates and how many votes each one has during tallying
    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    // If the election is open, the voter can proceed to the voter checkin
    if metadata.status == "open" {
        // Voter checkin asks for their name and birthdate
        println!("Voter checkin");
        println!("Enter your name: ");
        let votername = utils::read_input().to_lowercase();

        // The date of birth needs to be formatted correctly, this loop continues until it is entered in the correct format 
        let dob;
        loop {
            println!("Enter your birthdate (mm/dd/yyyy): ");
            let input = utils::read_input();

            // The regex expression sets a pattern that must be matched for the birthdate (mm/dd/yyyy)
            if Regex::new(r"^(0[1-9]|1[0-2])/(0[1-9]|[12][0-9]|3[01])/\d{4}$")?.is_match(&input) {
                dob = input;
                break;
            }
            else {
                println!("incorrect format")
            }
        }

        // Check voter registration to see if voter is registered and that they have not previously voted
        // If they are registered and have not cast a ballot, they can proceed to voting
        if is_eligible(&votername, &dob)? {
            clear()?;
            loop {
                // Get list of candidates in each office from metadata file
                let metadata_file = fs::File::open("./ballot/metadata.json")?;
                let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;
                let presidents = metadata.presidential_candidates;
                let senators = metadata.senate_candidates;
                let judges = metadata.judicial_candidates;

                // Display president candiates and get vote
                println!("Presidential Candidates:");
                let president_vote = match voting::present_candidates(&presidents) {
                    Ok(vote) => vote,
                    Err(e) => {
                        eprintln!("Error presenting presidential candidates: {}", e);
                        continue;
                    }
                };
                // Use vote to return candidate object for the one that was chosen
                let president_choice = match presidents.get(president_vote as usize) {
                    Some(choice) => choice,
                    None => {
                        eprintln!("President vote did not return a valid candidate.");
                        continue;
                    }
                };
            
                // Display senate candiates and get vote
                println!("Senate Candidates:");
                let senate_vote = match voting::present_candidates(&senators) {
                    Ok(vote) => vote,
                    Err(e) => {
                        eprintln!("Error presenting senate candidates: {}", e);
                        continue;
                    }
                };
                // Use vote to return candidate object for the one that was chosen
                let senate_choice = match senators.get(senate_vote as usize) {
                    Some(choice) => choice,
                    None => {
                        eprintln!("Senate vote did not return a valid candidate.");
                        continue;
                    }
                };

                // Display judicial candiates and get vote
                println!("Judicial Candidates:");
                let judge_vote = match voting::present_candidates(&judges) {
                    Ok(vote) => vote,
                    Err(e) => {
                        eprintln!("Error presenting judicial candidates: {}", e);
                        continue;
                    }
                };
                // Use vote to return candidate object for the one that was chosen
                let judge_choice = match judges.get(judge_vote as usize) {
                    Some(choice) => choice,
                    None => {
                        eprintln!("Judge vote did not return a valid candidate.");
                        continue;
                    }
                };

                // After selecting a candidate for each office, conform their choices, and if the voter 
                // wants to change them, restart voting 
                loop {
                    // Show voter what they selected and confirm
                    println!("Are these choices correct?");
                    println!("President:\t{}\t{}", president_choice.name, president_choice.party);
                    println!("Senate:\t\t{}\t{}", senate_choice.name, senate_choice.party);
                    println!("Judge:\t\t{}\t{}", judge_choice.name, judge_choice.party); 

                    print!("(y/n): ");
                    _ = io::stdout().flush();
                    let response = read_input();

                    // If yes, cast ballot with choices, and change flag in voter database to indicate that they voted and
                    // prevent future votes from this voter
                    if response.to_lowercase() == "y" {
                        if let Err(e) = voting::cast_ballot(president_vote, senate_vote, judge_vote) {
                            eprintln!("Failed to cast ballot: {}", e);
                            continue;
                        }
                        if let Err(e) = voting::change_to_voted(get_voter_index(&votername, &dob)?, &votername, &dob) {
                            eprintln!("Failed to update voter status: {}", e);
                        }
                        clear()?;
                        println!("Vote successfull recorded.");
                        return Ok(())
                    }
                    // Return to top of loop if voter doesn't accept their choices
                    else if response.to_lowercase() == "n" {
                        clear()?;
                        break;
                    }
                    else {
                        clear()?;
                        println!("Invalid input. Enter y or n.")
                    }
                }
                clear()?;
            } 
        }
        else {
            return Ok(())
        }
    }
    // If the election is closed, votes can't be accepted, and the program closes
    else if metadata.status == "closed" {
        println!("Election is currently closed.");
        return Ok(())
    }
    else {
        return Ok(())
    }
}


// This function contains the admin interface, leading to all admin functionality
fn admin_loop() -> Result<(), Box<dyn Error>> {
    println!("Admin Interface");

    loop {
        // Load metadata file containing information about the candidates and how many votes
        // each one has during tallying
        let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
        let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

        // If election is open, an admin can register new voters, close the election, or refresh the ballot
        if metadata.status == "open" {
            println!("Election is currently open\n");
            println!("Enter 1 to register new voters");
            println!("Enter 2 to close the election");
            println!("Enter 3 to create new election ballot");
            println!("Press enter to end session");

            print!("Selection: ");
            _ = io::stdout().flush();
            let selection = utils::read_input();

            // Enter 1 to register a new voter
            if selection == "1" {
                clear()?;
                if let Err(e) = admin::register_voters() {
                    eprintln!("Failed to register voter: {}", e);
                }
            }
            // Enter 2 to close the election
            else if selection == "2" {
                clear()?;
                metadata = match admin::close_election() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to close election: {}", e);
                        metadata
                    }
                };
            }
            // Enter 3 to refresh the ballot (clears candidates, deletes votes and log file)
            else if selection == "3" {
                clear()?;
                metadata = match admin::create_ballot() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to create ballot: {}", e);
                        metadata
                    }
                };
            }
            // Enter nothing to exit the admin interface and close the program
            else if selection == "" {
                return Ok(());
            }
            else {
                println!("Invalid selection");
            }
        }

        // If the election is closed, an admin can register new voters, open the election, add candidates, tally votes, or refresh the ballot
        else if metadata.status == "closed" {
            println!("Election is currently closed\n");
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

            // Enter 1 to register new voters
            if selection == "1" {
                clear()?;
                if let Err(e) = admin::register_voters() {
                    eprintln!("Failed to register voter: {}", e);
                }
            }
            // Enter 2 to open the election
            else if selection == "2" {
                clear()?;
                metadata = match admin::open_election() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to open election: {}", e);
                        metadata
                    }
                };
            }
            // Enter 3 to add a new candidate to the ballot
            else if selection == "3" { 
                clear()?;
                metadata = match admin::add_candidate() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to add candidate: {}", e);
                        metadata
                    }
                };
            }
            // Enter 4 to tally the votes and determine winners
            else if selection == "4" {
                clear()?;
                if let Err(e) = admin::tally_votes() {
                    eprintln!("Failed to tally votes: {}", e);
                }
                if let Err(e) = admin::declare_winners() {
                    eprintln!("Failed to declare winners: {}", e);
                }
                return Ok(())
            }
            // Enter 5 to refresh the ballot (clears candidates, deletes votes and log file)
            else if selection == "5" {
                clear()?;
                metadata = match admin::create_ballot() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to create ballot: {}", e);
                        metadata
                    }
                };
                println!("New election ballot created.\n");
            }
            // Enter nothing to exit the admin interface and close the program
            else if selection == "" {
                return Ok(());
            }
            else {
                clear()?;
                println!("Invalid selection");
            }
        }

        // Save any changes made to the metadata before returning to top of the loop
        let updated_metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write("./ballot/metadata.json", updated_metadata_json)?;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "admin" {
            clear().expect("failed to clear screen");

            match admin::admin_authenticate() {
                Ok(true) => {
                clear().expect("failed to clear screen");
                _ = admin_loop();
                }
                Ok(false) => {
                    println!("Authentication failed");
                }
                Err(e) => {
                    eprintln!("Error during authentication: {}", e);
                }
            }
        }
        else if args[1] == "dbg" {
            match admin::admin_authenticate() {
                Ok(true) => {
                    clear().expect("failed to clear screen");
                    _ = io::stdout().flush();
                    let selection = utils::read_input();
                    if selection == "1" {
                        dbg::testing_ballot();
                    }
                    else if selection == "2" {
                        dbg::testing_voter_reg();
                    }
                    else if selection == "3" {
                        dbg::testing_tally_votes();
                    }
                    else {}
                }
                Ok(false) => {
                    println!("Authentication failed");
                }
                Err(e) => {
                    eprintln!("Error during authentication: {}", e);
                }
            }
        }
        else {
            eprintln!("Unknown argument");
        }
    }
    else {
        _ = io::stdout().flush();
        _ = voter_loop();
    }
}
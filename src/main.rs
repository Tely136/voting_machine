mod admin;
mod voting;
mod utils;
mod dbg;
use std::{
    error::Error,
    env,
    fs,
    io::{self, Write},
};
use clearscreen::{self, clear};
use utils::read_input;
use voting::alread_voted;


fn voter_loop() -> Result<(), Box<dyn Error>> {
    println!("Welcome to the voting machine");

    let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
    let metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

    if metadata.status == "open" {
        println!("Voter checkin");
        println!("Enter your name: ");
        let votername = utils::read_input().to_lowercase();

        println!("Enter your birthdate (mm/dd/yyyy): ");
        let dob = utils::read_input();

        // check voter registration using name and birthdate
        let verification = voting::verify_voter_data(&"voter_db.csv", &votername, &dob)?;
        if verification.0 == true {
            if !alread_voted(&votername, &dob)? {
                clear()?;
                loop {
                    // Display presidential candiates and get vote
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
                    let judge_choice = match judges.get(judge_vote as usize) {
                        Some(choice) => choice,
                        None => {
                            eprintln!("Judge vote did not return a valid candidate.");
                            continue;
                        }
                    };

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
                            if let Err(e) = voting::cast_ballot(president_vote, senate_vote, judge_vote) {
                                eprintln!("Failed to cast ballot: {}", e);
                                continue;
                            }
                            if let Err(e) = voting::change_to_voted(verification.1, &votername, &dob) {
                                eprintln!("Failed to update voter status: {}", e);
                            }
                            clear()?;
                            println!("Vote successfull recorded.");
                            return Ok(())
                        }
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
                println!("You have already voted in this election");
                return Ok(())
            }
        }
        else {
            println!("Voter registration not found");
            return Ok(())
        }
    }
    else if metadata.status == "closed" {
        println!("Election is currently closed.");
        return Ok(())
    }
    else {
        return Ok(())
    }
}


fn admin_loop() -> Result<(), Box<dyn Error>> {
    println!("Admin Interface");

    loop {
        let metadata_file = fs::File::open(&"./ballot/metadata.json")?;
        let mut metadata: utils::ElectionMetadata = serde_json::from_reader(&metadata_file)?;

        if metadata.status == "open" {
            println!("Election is currently open");
            println!("");
            println!("Enter 1 to register new voters");
            println!("Enter 2 to close the election");
            println!("Enter 3 to create new election ballot");
            println!("Press enter to end session");

            print!("Selection: ");
            _ = io::stdout().flush();
            let selection = utils::read_input();

            if selection == "1" {
                clear()?;
                admin::register_voters();
            }
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
            else if selection == "" {
                return Ok(());
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
                clear()?;
                admin::register_voters();
            }

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
            else if selection == "5" {
                clear()?;
                metadata = match admin::create_ballot() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to create ballot: {}", e);
                        metadata
                    }
                };
                println!("New election ballot created.");
                println!("");
            }
            else if selection == "" {
                return Ok(());
            }
            else {
                clear()?;
                println!("Invalid selection");
            }
        }
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
            dbg::testing_ballot();
            dbg::testing_voter_reg();
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
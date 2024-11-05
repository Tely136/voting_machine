use std::{
    // env,
    error::Error, fs, io::{self, Write}, path, process
};
use csv;


fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });
    input.trim().to_string()
}


fn main() {
    loop {
        println!("Admin interface");
        println!("Enter 1 to create new ballot");
        println!("Enter 2 to add candidates to a ballot");
        println!("Enter 3 to register new voters");
        println!("Enter 4 to open or close an election");
        println!("Enter 5 to tally votes for an election");
        print!("Selection: ");
        _ = io::stdout().flush();
        let selection = read_input().trim().parse::<i32>().unwrap();

        if selection == 1 {
            // get folder name for ballot
            // save csv file with header for name, party, political office
            print!("Enter ballot name:");
            _ = io::stdout().flush();
            let folder_name = read_input();

            // check if folder exists
            let folder_path = path::Path::new("./").join(&folder_name);
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
        else if selection == 2 {
            // get folder name to load ballot from
            // create ballot object using the file
            // loop asking for new candidates to be added to ballot
            // end loop when certain input is entered 

            print!("Enter ballot name:");
            _ = io::stdout().flush();
            let folder_name = read_input();
            println!("");

            let folder_path = path::Path::new("./").join(&folder_name);
            if folder_path.try_exists().expect("couldn't check existence") == true { // need to fix the expect   
                loop {
                    println!("Enter 0 to finish");
                    print!("Enter candidate name: "); // need error for invalid input that forces user to retry
                    _ = io::stdout().flush();
                    let candidate_name = read_input();
                    if candidate_name == 0.to_string() {
                        break;
                    }

                    print!("Enter candidate party: ");
                    _ = io::stdout().flush();
                    let candidate_party = read_input();
                    if candidate_party == 0.to_string() {
                        break;
                    }

                    print!("Enter 1 for President, 2 for Senate, 3 for Judge: "); // need error for invalid input that forces user to retry
                    _ = io::stdout().flush();
                    let mut candidate_office = String::from("");
                    let candidate_office_input = read_input();
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
        else if selection == 3 {
            // open csv file of registered boters (maybe later this file can be encrypted or something idk)
            // loop asking fo user input for name and birthdate
            // end loop when certain input is entered
            
        }
        else if selection == 4 {
            
        }
        else if selection == 5 {
            
        }
        else {
            println!("Invalid selection");
        }
        println!("");
    }
}

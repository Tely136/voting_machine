use std::{
    // env,
    io::{self, Write}, error::Error, process
};
use csv;


struct Ballot {
    presidential_candidates: Vec<Candidate>,
    senate_candidates: Vec<Candidate>,
    judicial_candidates: Vec<Candidate> 
}

struct Candidate {
    name: String,
    party: String
}

impl Candidate {
    fn new() -> Candidate {
        print!("Enter candidate name: ");
        _ = io::stdout().flush();
        let name = read_input().trim().to_string();

        print!("Enter candidate party: ");
        _ = io::stdout().flush();
        let party = read_input().trim().to_string();

        Candidate { name, party }
    }

    fn from_file(name: &str, party: &str) -> Candidate {
        let name = name.to_string();
        let party = party.to_string();
        Candidate {name, party}
    }
}

impl Ballot {
    // change to save ballot to a file and 
    fn new(filepath: &str) {
        let mut wtr = csv::Writer::from_path(filepath).unwrap(); // fix the unwrap here
        wtr.write_record(&["Name", "Party", "Office"]).unwrap();
        wtr.flush().unwrap();
    }

    // load ballot from file
    fn load(filename: &str) -> Ballot {
        let mut presidential_candidates = vec![];
        let mut senate_candidates = vec![];
        let mut judicial_candidates = vec![];

        let mut rdr = csv::Reader::from_path(filename).unwrap();
        for result in rdr.records() {
            let record = result.unwrap();

            let candidate = Candidate::from_file(&record[0], &record[1]);

            if &record[2] == "President" {
                presidential_candidates.push(candidate);
            }
            else if &record[2] == "Senate" {
                senate_candidates.push(candidate);
            }
            else if &record[2] == "Judge" {
                judicial_candidates.push(candidate);
            }
            else {
                // return an error
            }
        }

        Ballot { presidential_candidates, senate_candidates, judicial_candidates}
    }

    fn save(filepath: &str, ballot: Ballot) {
        let mut wtr = csv::Writer::from_path(filepath).unwrap(); // fix the unwrap here
        wtr.write_record(&["Name", "State", "Office"]).unwrap();
        wtr.flush().unwrap();
        
        for candidate in ballot.presidential_candidates {
            wtr.write_record(&[candidate.name, candidate.party, "President".to_string()]).unwrap();

        }
    }

    fn add_candidates(mut ballot: Ballot) -> Ballot {
        
        println!("Enter 1 for president, 2 for senate, 3 for judicial");
        let office_val = read_input().trim().parse::<i32>().unwrap(); // fix the unwrap here and check for correct input

        if office_val == 1 {
            ballot.presidential_candidates.push(Candidate::new());
        }
        else if office_val == 2 {
            ballot.senate_candidates.push(Candidate::new());
        }
        else if office_val == 3 {
            ballot.judicial_candidates.push(Candidate::new());
        }
        else {
            println!("invalid input, try again");
        }
        ballot
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });

    input
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
            //Create new ballot then return to admin interface
            let test_ballot = Ballot::new("ballot.csv");
        }
        else if selection == 2 {
            // Add candidates until a certain value is entered
            let test_ballot: Ballot = Ballot::add_candidates(Ballot::load("ballot.csv"));
            Ballot::save("ballot.csv", test_ballot);

        }
        else if selection == 3 {
            
        }
        else if selection == 4 {
            
        }
        else if selection == 5 {
            
        }
        else {
            println!("Invalid selection");
        }
    }

    // let test_ballot = Ballot::new("test_ballot.csv");
    // let test_ballot: Ballot = Ballot::add_candidates(test_ballot);

    // dbg!(&test_ballot.presidential_candidates[1].name);

}

mod admin;
mod utils;
mod voter;

use std::{
    // env,
    // error::Error,
    // fs,
    io::{self, Write},
    // path,
    // process
};
use std::error::Error;
use csv::{ReaderBuilder, StringRecord};
use std::fs::OpenOptions;


fn voter_loop() {
    loop {
       println!("Voter Verify");

       print!("Enter Your Name: ");
       _ = io::stdout().flush();
       let votername = utils::read_input();
   
       print!("Enter Date of Birth: ");
       _ = io::stdout().flush();
       let dob = utils::read_input();

  let verify = voter::verify_voter_data(&"voter_db.csv", &votername, &dob).unwrap();
  if verify == true{
    println!("Voter Already Exists");
  }
  else{println!("Voter Doesn't Exist");
  }
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
        println!("***Voter Options***");
        println!("Admins enter 0 to login");
        voter_loop();
        _ = io::stdout().flush();
        let input = utils::read_input();
        if input == "0" {
            // if admin_authenticate == true {
                admin_loop();
            // }
        }
    }
}
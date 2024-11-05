use std::io::{self, Write};

fn main() {
    print!("Enter username: ");
    _ = io::stdout().flush();
    let input_username = read_input().trim().to_string();

    print!("Enter password: ");
    _ = io::stdout().flush();
    let input_password = read_input().trim().to_string();

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

// Function to read user input from the terminal
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



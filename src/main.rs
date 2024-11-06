use std::io::{self, Write};
use std::fs::OpenOptions;
use csv::WriterBuilder;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });
    input.trim().to_string()  // Trim the newline and return the input
}

fn main() {
    print!("Enter Your Name: ");
    _ = io::stdout().flush();
    let votername = read_input();

    print!("Enter Date of Birth: ");
    _ = io::stdout().flush();
    let dob = read_input();

    // Append the data to voter_db.csv`
    if let Err(e) = append_to_csv("voter_db.csv", &votername, &dob) {
        eprintln!("Failed to write to CSV: {}", e);
    } else {
        println!("Data saved successfully!");
    }
}

fn append_to_csv(file_path: &str, name: &str, dob: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.write_record(&[name, dob])?;
    writer.flush()?;
    Ok(())
}
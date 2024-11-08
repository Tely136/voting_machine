use std::{
    // env,
    // error::Error,
    // fs,
    io
    // path,
    // process
};
use std::fs::OpenOptions;
use csv::WriterBuilder;

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });
    input.trim().to_string()
}

pub fn append_to_csv(file_path: &str, name: &str, dob: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    writer.write_record(&[name, dob])?;
    writer.flush()?;
    Ok(())
}
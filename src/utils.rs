use std::{
    // env,
    // error::Error,
    // fs,
    io
    // path,
    // process
};

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
use std::fs::File;
// use std::io::{self, prelude::*};
use std::io::{self, BufReader, BufRead};
// use std::fs;

fn read_username(path: &str) -> Result<String, io::Error> {
    // fs::read_to_string(path)
    let mut username = String::new();

    let file = File::open(path)?;

    // let mut buf_reader = BufReader::new(file);

    BufReader::new(file).read_line(&mut username)?;

    Ok(username.trim().to_string())
}

fn main() {
    match read_username("D:\\Desktop\\Formations\\Rust\\rust-book-projects\\exercises\\safe_file_reader\\src\\names.txt") {
        Ok(username) => println!("The name you are waiting for is : {}", username),
        Err(error) => eprintln!("Failed to read the username: {error:?}"),
    }
}

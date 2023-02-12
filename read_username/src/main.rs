use std::fs::File;
use std::io::{self, Read};

fn main() {
    let username = match read_username_from_file("username") {
        Ok(username) => username,
        Err(e) => panic!("failed to open file: {:?}", e),
    };

    println!("username: {}", username)
}

fn read_username_from_file(file: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(file)?.read_to_string(&mut username)?;
    Ok(username)
}

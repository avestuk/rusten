use std::{
    fs::{read_to_string, File, OpenOptions},
    io::{Read, Write},
};

fn main() {
    // File::open opens a file RO so this is somewhat of a silly example
    // Kinda cool that I figured out how to get the match arms to match up
    //let greeting_file_result = File::open("hello.txt");

    // This works however! Open a file RW and create if it's not found
    let greeting_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("hello.txt");

    let mut greeting_file = greeting_file_result.unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").expect("Problem creating file")
        } else {
            panic!("failed to read file: {:?}", error)
        }
    });

    let mut contents = String::new();
    greeting_file
        .read_to_string(&mut contents)
        .expect("Failed to read file: ");

    println!("File already exists, contents: {contents}");

    // Using closures as above can mae the logic much neater
    // let mut greeting_file = match greeting_file_result {
    //     Ok(mut file) => {
    //         let mut contents = String::new();
    //         match file.read_to_string(&mut contents) {
    //             Ok(_) => println!("File already exists, contents: {contents}"),
    //             Err(e) => panic!("Failed to read file: {:?}", e),
    //         }
    //         file
    //     }
    //     Err(error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    match greeting_file.write_all(String::from("hello m9").as_bytes()) {
        Ok(_) => println!("Wrote stuff"),
        Err(error) => panic!("Failed to write: {:?}", error),
    };

    let k = fs::read_to_string();
}

fn read_from_file(file: &String) -> Result<String, std::io::Error> {
    let mut file_contents = String::new();
    File::open(file)?.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

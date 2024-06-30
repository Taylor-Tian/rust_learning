use std::fs::File;
use std::io::{self,Read};

fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_content("hello.txt") {
        Ok(content) => println!("file content: {}", content),
        Err(err) => println!("Error reading file: {}", err),
    }
}
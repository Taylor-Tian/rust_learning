use std::num::ParseIntError;
use std::fs;

#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseIntError(ParseIntError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "I/O error: {}", err),
            AppError::ParseIntError(err) => write!(f, "Parse int error: {}", err),
        }
}
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ParseIntError(err)
    }
}

fn read_number_from_file() -> Result<i32, AppError> {
    let content = fs::read_to_string("numbers.txt")?;
    let num: i32 = content.trim().parse()?;
    Ok(num)
}

fn main() {
    match read_number_from_file() {
        Ok(num) => println!("The number read from the file is: {}", num),
        Err(err) => eprintln!("Error reading the file: {}", err),
    }
}
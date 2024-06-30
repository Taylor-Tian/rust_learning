use std::fmt;

#[derive(Debug)]
enum MyError {
    InvalidInput,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Invalid input")
    }
}

fn validate_number(num: i32) -> Result<(), MyError> {
    if num > 0 {
        Ok(())
    } else {
        Err(MyError::InvalidInput)
    }
}

fn main() {
    match validate_number(-1) {
        Ok(_) => println!("Number is valid"),
        Err(err) => println!("Error: {}", err),
    }
}
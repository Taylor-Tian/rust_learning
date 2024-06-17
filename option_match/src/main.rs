fn main() {
    let some_number = parse_number("42");
    let no_number = parse_number("abc");

    match some_number {
        Some(value) => println!("Parsed number: {}", value),
        None => println!("Could not parse number"),
    }

    match no_number {
        Some(value) => println!("Parsed number: {}", value),
        None => println!("Could not parse number"),
    }
}

fn parse_number(s: &str) -> Option<i32> {
    match s.parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}


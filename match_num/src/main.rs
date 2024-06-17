fn main() {
    for number in 1..10 {
        println!("The word for {} is {}", number,number_to_word(number));
    }
}

fn number_to_word(number:u32) -> &'static str {
    match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        _ => "unknown",
    }
}
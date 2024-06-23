fn number_to_string(number: i32) -> &'static str {
    match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "Other",
    }
}

fn main() {
    let num = 3;
    let result = number_to_string(num);
    println!("The number{} is spelled as {}",num, result);
}
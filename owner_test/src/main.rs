fn main() {
    let s = String::from("hello");
    let len = print_and_return_length(s.clone());
    println!("The length of {} is {}.",s, len);
}

fn print_and_return_length(s: String) -> usize {
    println!("{}", s);
    s.len()
}

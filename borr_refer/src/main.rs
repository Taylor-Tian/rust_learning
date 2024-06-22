fn main() {
    let s = String::from("hello");
    let len = calculate_len(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

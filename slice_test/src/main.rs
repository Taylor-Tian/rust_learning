fn main() {
    let s = String::from("hello world");
    let (first, second) = split_at_space(&s);
    println!("{}, {}", first, second);
}

fn split_at_space(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return (&s[0..i], &s[i+1..s.len()]);
        }
    }
    (&s[..],&s[..])
}

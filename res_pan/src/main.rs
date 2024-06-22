fn count_charaters(s: &str) -> usize {
    if s.is_empty() {
        panic!("The string is empty");
    } else{
        s.chars().count()
    }
}


fn main() {
    let s = "hello";
    let count = count_charaters(s);
    println!("the number of {} is {}",s , count);
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(", Rustaceans");
    let s3 = s1 + &s2;
    println!("Combined string:{}", s3);

    let slice = &s3[0..5];
    println!("Sliced string:{}", slice);
}
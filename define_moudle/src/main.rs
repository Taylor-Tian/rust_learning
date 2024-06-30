fn main() {
    let number = Some(7);

    match number {
        Some(x) if x % 2==0 => println!("Even number: {}", x),
        Some(x) => println!("Odd number: {}", x),
        None => println!("No number provided"),
    }
}
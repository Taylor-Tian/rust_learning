fn main() {
    greet();
    let result = add(5,3);
    println!("The sum is: {}", result);
}

fn greet() {
    println!("Hello, world!");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

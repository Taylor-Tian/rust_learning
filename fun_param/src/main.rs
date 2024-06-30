fn main() {
    let s = String::from("Rust");
    takes_ownership(s.clone());
    println!("{}", s); // Error: s is moved into takes_ownership

    let x = 42;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}
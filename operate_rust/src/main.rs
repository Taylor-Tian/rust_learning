fn main() {
    let a = 10;
    let b = 20;

    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);
    println!("{} % {} = {}", a, b, a % b);
    println!("{} == {} = {}", a, b, a == b);
    println!("{}!= {} = {}", a, b, a!= b);
    println!("{} > {} = {}", a, b, a > b);
    println!("{} < {} = {}", a,b, a < b);
    println!("{} >= {} = {}", a, b, a >= b);
    println!("{} <= {} = {}", a, b, a <= b);

    let c = true;
    let d = false;
    println!("{} && {} = {}", c, d, c && d);
    println!("{} || {} = {}", c, d, c || d);
    println!("!{} = {}", c,!c);
}

fn main() {
    let mut count = 0;

    loop {
        count += 1;
        if count == 10 {
            break;
        }
    }
    println!("Loop stopped at count: {}", count);
}

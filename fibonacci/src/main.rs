fn main() {
    let n = 10;
    for i in 0..n {
        println!("Fibonacci {}: {}", i,fibonacci(i));
    }
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let some_value: Option<i32> = Some(10);

    if let Some(value) = some_value {
        println!("Found a Value: {}", value);
    }

    let mut values = vec![Some(1),Some(2),Some(3)];

    while let Some(Some(v)) = values.pop() {
        println!("Popped a value: {}", v);
    }
}
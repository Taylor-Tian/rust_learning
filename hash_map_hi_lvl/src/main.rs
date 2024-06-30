use std::collections::HashMap;

fn main() {
    let mut scores =HashMap::new();
    scores.insert("Apple", 3);
    scores.insert("banana", 2);

    if let Some(score) = scores.get("Apple") {
        println!("Count of apple: {}", score);
    }

    for (key, score) in &scores {
        println!("{}: {}", key, score);
    }
}
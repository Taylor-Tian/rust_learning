use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("orange");

    if set.contains("apple") {
        println!("The set contains apple!");
    } else {
        println!("The set does not contain apple");
    }
}
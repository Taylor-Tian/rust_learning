extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person { name: String::from("Alice"), age: 30 };
    let json = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", json);

    let deserialized: Person = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {} is {} years old", deserialized.name, deserialized.age);
}

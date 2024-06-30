fn main() {
    let mut vec = vec![10, 20, 30];
    vec.push(40);
    println!("Vector: {:?}", vec);

    vec.pop();
    println!("Vector after pop: {:?}", vec);

    for i in &vec {
        println!("Element: {}", i);
    }
}
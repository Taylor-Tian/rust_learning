#[derive(Debug)]
struct Car {
    brand: String,
    year: u32,
}

fn main() {
    let my_car = Car {
        brand: String::from("Toyota"),
        year: 2021,
    };

    println!("{}{}", my_car.brand,my_car.year); // Output: Car { brand: "Toyota", year: 2020 }
}
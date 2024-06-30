enum Vehicle {
    Car { brand: String, year: u16},
    Bicycle(String),
}

fn main() {
    let my_vehicle = Vehicle::Car { brand: String::from("Toyota"), year: 2020};

    match my_vehicle {
        Vehicle::Car {brand, year} => println!("Car: {} made in {}",brand, year),
        Vehicle::Bicycle(brand) => println!("Bicycle brand: {}", brand),
    }
}
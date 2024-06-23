struct Rectangle {
    width: u32,
    height: u32,
}

enum Shape {
    Circle(f64),
    Rectangle { width: u32, height: u32},
}

fn main () {
    let rect = Rectangle {width: 30, height: 50 };
    let shape = Shape::Rectangle {width: 10, height: 20};
    let circle = Shape::Circle(10.0);

    match rect {
        Rectangle {width, height} => println!("Rectangle with width {} and height {}.", width,height),
    }

    match shape {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle {width, height} => println!("Rectangle with width {} and height {}.", width,height),
    }
    match circle {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle {width, height} => println!("Rectangle with width {} and height {}.", width,height),
    }
}
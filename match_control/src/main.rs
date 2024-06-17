enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn main(){
    let dirctions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    for dirction in &dirctions {
        match dirction {
            Direction::Up => println!("Going Up"),
            Direction::Down => println!("Going Down"),
            Direction::Left => println!("Going Left"),
            Direction::Right => println!("Going Right"),
        }
    }
}
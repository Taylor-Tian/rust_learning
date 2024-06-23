use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

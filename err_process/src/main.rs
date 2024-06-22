use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f= File::open("Hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Error creating file: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("Problem opening file: {:?}", error)
        },
    };
}

use std::fs::File;
use std::io::ErrorKind;
fn main() {
    println!("Hello, world!");
    let f = File::open("Hello.txt");

    let f = match f{
        Ok(file) => file,
        Err (ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("Hello.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                        )
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

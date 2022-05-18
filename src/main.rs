use std::fs::{File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let age = "16".to_string();

    let age: u32 = match age.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };

    match age {
        16 => println!("Yes!"),
        c => println!("But: {}", c),
    }

    let my_file = match File::open("./parthka") {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Creating ./parthka file");
                let p;
                if 1 == 1 {
                    println!("123");
                    p = File::create("./parthka").unwrap();
                } else {
                    p = File::create("./parthka").unwrap();
                }
                p
            }
            _ => std::process::exit(0),
        },
    };

    let mut text1 = std::fs::read_to_string("./parthka").expect("Something went wrong reading the file");;
    println!("{:?}", my_file);
    println!("{}", text1);
}

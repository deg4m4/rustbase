use mystr::strfn::*;
use std::env;
use std::error::Error;
use std::fs;
use std::string::ToString;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let l = match Args::get_arg(&arg) {
        Ok(n) => n,
        Err(_e) => panic!("Sorry, {}", _e),
    };

    let envvl = match env::var("CONF_RLINE") {
        Ok(s) => s,
        Err(_e) => "".to_string(),
    };

    if envvl == "true".to_string() {
        println!("{}", envvl);
    }

    let p = l.print_text();

    if envvl == "true".to_string() {
        println!("{}", p.unwrap());
        println!("{}", "Thanks For One Time...")
    }
}

struct Args<T> {
    query: T,
    file: T,
}

trait ArgsFunc<T> {
    fn get_arg(a: &Vec<T>) -> Result<Args<&T>, String>;
    fn print_text(&self) -> Result<String, Box<dyn Error>>;
}

impl<T> ArgsFunc<T> for Args<T>
where
    T: std::fmt::Display + std::convert::AsRef<std::path::Path> + ToString,
{
    fn get_arg(a: &Vec<T>) -> Result<Args<&T>, String> {
        if a.len() < 3 {
            return Err(format!("{}", "Give Three Or More Arguments..."));
        }
        let file = &a[2];
        let query = &a[1];
        Ok(Args { query, file })
    }

    fn print_text(&self) -> Result<String, Box<dyn Error>> {
        println!("{}", self.file);
        println!("{}", self.query);
        let file = match fs::read_to_string(&self.file) {
            Ok(c) => c,
            Err(_e) => "Not File Found".to_string(),
        };
        let mut p = 0;
        for l in search(&self.query.to_string(), &file.to_string()) {
            println!("Line {}: {}", p, l);
            p += 1;
        }

        Ok(String::from("Success"))
    }
}

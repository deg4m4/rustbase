use std::env;
use std::fs;
use std::error::Error;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let l = match Args::get_arg(&arg) {
        Ok(n) => n,
        Err(_e) => panic!("Sorry, {}", _e),
    };

    if let Err(e) = l.print_text(){
        println!("{}", e);
    }

    let p = l.print_text();
    println!("{}", p.unwrap());

    println!("{}", "Thanks For One Time...")
}

struct Args<T> {
    name: T,
    file: T,
}

trait ArgsFunc<T> {
    fn get_arg(a: &Vec<T>) -> Result<Args<&T>, String>;
    fn print_text(&self) ->  Result<String, Box<dyn Error>>;
}

impl<T> ArgsFunc<T> for Args<T>
where T: std::fmt::Display + std::convert::AsRef<std::path::Path>
{
    fn get_arg(a: &Vec<T>) -> Result<Args<&T>, String> {
        if a.len() < 3 {
            return Err(format!("{}", "Give Three Or More Arguments..."));
        }
        let file = &a[2];
        let name = &a[1];
        Ok(Args { name, file })
    }

    fn print_text(&self) -> Result<String, Box<dyn Error>>{
        println!("{}", self.file);
        println!("{}", self.name);
    
        let file = match fs::read_to_string(&self.file) {
            Ok(c) => c,
            Err(_e) => "Not File Found".to_string(),
        };
    
        println!("{}", file);
        Ok(String::from("Success"))
    }
}

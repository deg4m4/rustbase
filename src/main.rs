use std::env;
use std::fs;

fn main() {
    let arg: Vec<String> = env::args().collect();

    let f = Args{file: "".to_string(), name: "".to_string(), n:12};

    let s: Args<i32> = f.get_arg(&arg);

    let file = match fs::read_to_string(f.file) {
        Ok(c) => c,
        Err(_e) => "Not File Found".to_string(),
    };

    println!("{}", file);
}

struct Args<T> {
    name: T,
    file: T,
    n: T
}

trait ArgsFunc<T> {
    fn get_arg(&self, a: &Vec<String>) -> Args<&T>;
}

impl<T> ArgsFunc<T> for Args<T> 
{
    fn get_arg(&self, a: &Vec<String>) -> Args<&T> {
            Args { name: String::from("wdasd"), file: String::from("wdasd"), n: String::from("wdasd") }
        }
}

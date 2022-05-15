use std::io;

pub fn get() -> u32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input Error");
    match input.trim().parse::<u32>(){
        Ok(n) => n,
        Err(_e) => {
            println!("Please Enter Only Numeric Value!");
            get()
        }
    }
}
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
   
    println!("Guess the number!");

    let sec_num = rand::thread_rng().gen_range(1..101);

    println!("Shhu! Secret Number {}", sec_num);

    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Your Guessing is {}", guess.trim());

    let guess = guess.trim().parse::<i32>().expect("Please type a number!");

    match guess.cmp(&sec_num) {
        Ordering::Less => {
            println!("Lesser");
        }
        Ordering::Equal => {
            println!("Equaled");
        }
        Ordering::Greater => {
            println!("Greater");
        }
    }

}

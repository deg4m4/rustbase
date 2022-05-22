use mymm::*;
use mymm::guess::*;
use std::collections::HashMap;

fn main() {

    let n1 = "pkk";
    let result;
    let n2 = "pka";
    result = longest(&n1, &n2);  
    println!("Longest: {}", result);
    println!("{}", add());
    let g1 = Guess {
        num1 :"r",
        num2 :"r",
    };
    println!("{}", g1.big());
    println!("{}", g1.check());

    let mut p = HashMap::new();

    p.insert("name", "parth");
    p.insert("username", "parthka");

    let name = match p.get("name"){
        Some(n) => n,
        None => "1234"
    };

    println!("{}", name);

}

fn longest<'ab, T>(n1: &'ab T, n2: &'ab T) -> T
where T: PartialOrd + Copy
{
    if n1 > n2 {
        *n1
    } else {
        *n2
    }
}

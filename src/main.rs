use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
enum Name{
    Mr(&'static str),
    Ms(&'static str)
}

#[derive(Debug)]
enum Age<T>{
    Age(T)
}

#[derive(Debug)]
struct Person{
    name: Name,
    age: Age<u32>
}

impl Name{
    fn get_name(&self) -> &'static str {
        match self {
            Name::Mr(name) => Box::leak(format!("Mr: {}", name).into_boxed_str()),
            Name::Ms(name) => Box::leak(format!("Ms: {}", name).into_boxed_str()),
            _ => "pak"
        }
    }
}

impl Person{
    fn entry(name: &'static str, age: u32, male: bool) -> Person {
        Person{

            name: if male {
                Name::Mr(name)
            } else {
                Name::Ms(name)
            },
            age: Age::Age(age)
        }
    }
}

impl Person {
    fn id(&self) {
        println!("{}", self.name.get_name());
        println!("Age: {}", if let Age::Age(v) = self.age { v } else {0})
    }
}

fn main() {

    let pn1 = Name::Mr(("Pka"));
    let pa1 = Age::Age(17);
    let (p1) = (Person{name: pn1, age: Age::Age(17)});

    p1.id();

}
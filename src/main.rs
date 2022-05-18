use std::collections::HashMap;

fn main() {

    let mut person = HashMap::new();
    person.insert(String::from("name"), String::from("Parthka"));
    person.insert(String::from("username"), String::from("meparthka"));
    person.insert(String::from("age"), String::from("17"));
    person.insert(String::from("phone"), String::from("+91 886 688 1066"));
    person.entry(String::from("color")).or_insert("Dark-Light".to_string());

    for w in "I Am Parth".split_whitespace() {
        let c = person.entry(w.to_string()).or_insert("123".to_string());
        println!("{}", c);
    }

    println!("{:?}", person);

}
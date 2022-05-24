use mystr::strfn::search;

fn main() {
    let contents = "\
    Rust:
    safe, fast, parth.
    Pick three.
    parth Degama
    ";
    let l = search("Deg", contents);
    println!("Line: {}", l[0]);
}

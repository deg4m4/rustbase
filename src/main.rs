fn main() {

    for c in "parth".bytes() {
        println!("{}: {}", String::from_utf8_lossy(&[c]), c);
    }

}

#[allow(dead_code)]
fn pushing(p: &mut String) {
    p.push_str(" pop");
}
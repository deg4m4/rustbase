fn main() {

    let mut w: Vec<i32> = Vec::new();
    let mut x: [i32; 2] = [1, 3];
    x[0] = 123;
    let mut v = vec![12, 33, 34, 43, 233];
    w.push(123);
    w.push(123);
    w.push(123);
    w.push(12);

    let p: i32 = 33;

    match v.get(3) {
        Some(43) => println!("Yes It Is {}", Some(p).unwrap()),
        Some(33) => println!("Yes It Is but suiting {}", Some(p).unwrap()),
        _=>println!("No It Is")
    }

    for e in &v{
        println!("{}", e)
    }

    for e in &mut v{
        *e += 100;
    }

    for e in &v {
        println!("{}", e)
    }

}
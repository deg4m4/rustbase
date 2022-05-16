use std::io;

#[derive(Debug)]
struct Cube{
    h: u32,
    w: u32,
    b: u32,
}

fn main() {
    let mut b1: Cube = Cube{h:0, w:0, b:0};
    println!("{:?}", b1);
    b1.h = 7;
    b1.w = 5;
    b1.b = 10;
    println!("{:?}", b1);
    println!("Cube: {}", b1.cube())
}

impl Cube {
    fn cube(&mut self) -> u32{
        self.h = 5;
        dbg!(&self);
        self.b * self.h * self.w
    }
}



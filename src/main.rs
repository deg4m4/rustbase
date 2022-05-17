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
    println!("Cube: {}", b1.cube());

    let b2 = Cube::cube(&mut b1);
    dbg!(&b2);

    let mut b3 = Cube::navu(3, 4, 3);
    dbg!(&b3);
    println!("{}", b3.cube())
}

impl Cube {

    fn navu(h: u32, w:u32, b: u32) -> Cube{
        Cube{
            w,
            b,
            h
        }
    }

    fn cube(&mut self) -> u32{
        dbg!(&self);
        self.b * self.h * self.w
    }
}



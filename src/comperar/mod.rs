mod com;

use std::cmp::Ordering;
pub use com::equal;
use crate::tool::get;

pub fn min(num: u16, win: &mut i32, out: &mut i32) {
    println!("Please Enter The Minimum Number than {}", num);
    let num = num.try_into().unwrap();
    let inp = get();
    if inp.cmp(&num) == Ordering::Less {
        println!("You Are Win!");
        *win += 1;
    }else {
        println!("You Are Out!");
        *out += 1;
    }
}

pub fn max(num: u16, win: &mut i32, out: &mut i32) {
    println!("Please Enter The Maximum Number than {}", num);
    let num = num.try_into().unwrap();
    let inp = get();
    if inp.cmp(&num) == Ordering::Greater {
        println!("You Are Win!");
        *win += 1;
    }else {
        println!("You Are Out!");
        *out += 1;
    }
}
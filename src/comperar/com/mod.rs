use std::cmp::Ordering;
use crate::tool::get;

pub fn equal(num: u16,  win: &mut i32, out: &mut i32) {
    println!("Please Enter The Equal Number than {}", num);
    let num = num.try_into().unwrap();
    let inp = get();
    if inp.cmp(&num) == Ordering::Equal {
        println!("You Are Win!");
        *win += 1;
    }else {
        println!("You Are Out!");
        *out += 1;
    }
}
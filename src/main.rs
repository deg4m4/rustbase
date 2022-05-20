mod modal;

use modal::{Location, LocInfo}; 

fn main() {
    let p = Location {
        x_axis: 32,
        y_axis: 54.58,
        tree_age: vec![12, 43, 45, 21, 21],
    };
    println!("X Axis: {}", p.get_x());
    println!("Y Axis: {}", p.get_y());
    p.print_y();
    p.print_x();
    println!("Max Age: {}", p.get_max_age());
    let p = Location {
        x_axis: 32,
        y_axis: 43,
        tree_age: vec!['a', 'f', '😻', 'y', 'm'],
    };
    println!("X Axis: {}", p.get_x());
    println!("Y Axis: {}", p.get_y());
    p.print_y();
    p.print_x();
    let m: char = *p.get_max_age();
    println!("{}", m);
}

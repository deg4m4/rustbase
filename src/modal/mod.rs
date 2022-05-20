pub struct Location<X, Y, T> {
    pub x_axis: X,
    pub y_axis: Y,
    pub tree_age: Vec<T>,
}

pub trait LocInfo<X, Y, T> {
    fn get_x(&self) -> &X;
    fn get_y(&self) -> &Y;
    fn print_x(&self);
    fn print_y(&self);
    fn get_max_age(&self) -> &T;
}

impl<X, Y, T> LocInfo<X, Y, T> for Location<X, Y, T>
where
    X: std::fmt::Display + Copy,
    Y: Copy + std::fmt::Display,
    T: std::cmp::PartialOrd
{
    fn get_x(&self) -> &X {
        &self.x_axis
    }
    fn get_y(&self) -> &Y {
        &self.y_axis
    }
    fn print_x(&self) {
        println!("{}", self.x_axis);
    }
    fn print_y(&self) {
        println!("{}", self.y_axis);
    }
    fn get_max_age(&self) -> &T {
        let mut m = &self.tree_age[0];
        for n in &self.tree_age {
            if  n > m {
                m = n;
            }
        }
        m
    }
}
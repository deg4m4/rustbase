pub struct Guess<T> {
    pub num1: T,
    pub num2: T,
}

pub trait GuessMethod<T> {
    fn check(&self) -> bool;
    fn big(&self) -> &T;
}

impl<T> GuessMethod<T> for Guess<T>
where T: PartialEq + PartialOrd
{
    fn check(&self) -> bool {
        self.num1 == self.num2
    }

    fn big(&self) -> &T {
        if self.num1 > self.num2 {
            &self.num1
        } else {
            &self.num2
        }
    }
}
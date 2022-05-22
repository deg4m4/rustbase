pub mod guess;

pub fn add() -> i32 {
    74
}

pub fn add_2() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::*;
    use guess::*;
    #[test]
    fn it_works() {
        assert_eq!(add(), 74);
    }

    #[test]
    fn get_result() {
        assert!(!add_2());
    }

    #[test]
    fn check() {
        let mut p = Guess{
            num1: "12",
            num2: "44"
        };
        p.num1 = "44";
        assert!(p.check());
    }

    #[test]
    fn check_big() {
        let mut p = Guess{
            num1: 12,
            num2: 44
        };
        println!("123");
        assert_eq!(*p.big(), 21);
    }
    
}


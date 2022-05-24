pub mod strfn;

#[cfg(test)]
mod tests {

    use crate::strfn::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn find() {
        let contents = "\
Rust:
safe, fast, productive.
Pick threa.";
        assert_eq!(vec!["Pick three."], search("a", contents));
    }
}

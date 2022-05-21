fn main() {

    let n1 = "pkk";
    let result;
    let n2 = "pka";
    result = longest(&n1, &n2);  
    println!("Longest: {}", result);

}

fn longest<'ab, T>(n1: &'ab T, n2: &'ab T) -> T
where T: PartialOrd + Copy
{
    if n1 > n2 {
        *n1
    } else {
        *n2
    }
}

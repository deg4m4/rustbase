pub fn search<'a>(q: &'a str, s: &'a str) -> Vec<&'a str> {
    let mut ls = Vec::new();
    for l in s.lines() {
        if l.contains(q) {
            ls.push(l);
        }
    }
    ls
}

struct Add<T, U> {
    m:U,
    l:T,
}

impl<T, U> Add<T, U>{
    fn mgni(&self) -> &T {
        &self.l
    }

    fn leti(&self) -> &U {
        &self.m
    }
}

fn main() {

    let p = Add{m:122.3, l:43};

    println!("{}", p.mgni());
    println!("{}", p.leti());

}

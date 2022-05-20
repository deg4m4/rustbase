pub trait Summary {
    fn get_info(&self) -> String;
    fn summarize(&self) -> String {
        format!("I Am Self Depend")
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl NewsArticle {
    fn summarize2(&self) -> String {
        format!("Name: {}", self.author)
    }
}

impl Summary for NewsArticle {
    fn get_info(&self) -> String {
        format!("More Info To go @Blog {}", self.summarize())
    }
}

impl Summary for Tweet {
    fn get_info(&self) -> String {
        format!("More Info To go @Tweet {}", self.summarize())
    }
}

fn main() {
    let blog1 = NewsArticle {
        headline: "I Am Blog?".to_string(),
        location: "Internet".to_string(),
        author: "W3C".to_string(),
        content: "Relly, I Am Blog".to_string(),
    };

    println!("{:?}", blog1);
    println!("{}", blog1.get_info());

    let t1 = Tweet {
        reply: true,
        retweet: false,
        username: "Parthka".to_string(),
        content: "Relly, I Am Blog".to_string(),
    };

    println!("{:?}", t1);
    println!("{}", t1.get_info());

    println!("{}", get_my_author_name(&blog1));
    println!("{}", get_my_author_name(&t1));

    let nums = vec![45, 46, 58, 25, 74, 11];
    println!("{}", get_max(&nums));
    let names = vec!['p', 'a', 'r', 't', 'h'];
    println!("{}", get_max(&names));

}


fn get_my_author_name<U: Summary>(a: &U) -> String{
    format!("{}", a.get_info())
}

fn get_max<T: PartialOrd + Copy>(v: &[T]) -> T{
    let mut l = v[0];
    for &e in v {
        if e > l {
            l = e;
        }
    }
    l
}

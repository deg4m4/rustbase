pub trait Homo {
  type Name;
  type Item2;
  fn set_name(&self, name: Self::Name) -> Option<Self::Name>;
  fn println(&self, name: Self::Name);
}

#[derive(Clone)]
struct Homesepins {
  name: String,
}

impl Homo for Homesepins {
  type Name = &'static str;
  type Item2 = String;
  fn set_name(&self, name: Self::Name) -> Option<Self::Name> {
    self.println("123");
    self.println(name);
    Some(name)
  }
  fn println(&self, name: Self::Name) {
    println!("{}", name);
  }
}

#[derive(Clone)]
struct Monkey {
  name: String,
}

impl Homo for Monkey {
  type Name = i32;
  type Item2 = String;
  fn set_name(&self, name: i32) -> Option<i32> {
    self.println(32);
    self.println(name);
    Some(name)
  }
  fn println(&self, name: Self::Name) {
    println!("{}", name);
  }
}

trait HomeTrait<T> {
  fn set_name(&self, name: T) -> Option<T>;
  fn println(&self, name: T);
}

struct HomoStuct {}

impl<T> HomeTrait<T> for HomoStuct
where
  T: std::fmt::Display + Copy,
{
  fn set_name(&self, name: T) -> Option<T> {
    self.println("123");
    self.println(name);
    Some(name)
  }
  fn println(&self, name: T) {
    println!("{}", name);
  }
}

type hs = HomoStuct;

fn main() {
  let p1 = Homesepins {
    name: "parth".to_string(),
  };
  p1.set_name("asdasd");

  let p1 = Monkey {
    name: "parth".to_string(),
  };
  p1.set_name(4267);

  let p1 = hs {};
  p1.set_name("asdasd");

  let p1 = hs {};
  p1.set_name(2123123);

  let p1 = HomoStuct {};
  p1.set_name(87.6544);

  HomeTrait::set_name(&p1, 746864654.123);

}

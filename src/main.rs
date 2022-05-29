#[derive(Clone)]
enum address {
  address(Box<Node>),
  Nil,
}

#[derive(Clone)]
struct Node {
  value: i32,
  next: address,
}

impl Node {
  fn push(&mut self, value: i32) {
    match self.next {
      address::address(ref mut next) => {
        next.push(value);
      }
      address::Nil => {
        self.next = address::address(Box::new(Node {
          value,
          next: address::Nil,
        }));
      }
    }
  }

  fn push_node(&mut self, node: Node) {
    match self.next {
      address::address(ref mut next) => {
        next.push_node(node);
      }
      address::Nil => {
        self.next = address::address(Box::new(node));
      }
    }
  }

  fn print(&mut self) {
    println!("{}", self.value);
    match self.next {
      address::address(ref mut next) => {
        next.print();
      }
      address::Nil => {}
    }
  }

  fn delete(&mut self, v: i32) {
    match self.next {
      address::address(ref mut next_add) => {
        if next_add.value == v {
          self.next = next_add.next.clone();
        } else {
          next_add.delete(v);
        }
      }
      address::Nil => {}
    }
  }
}

fn main() {
  let mut n = Node {
    value: 23,
    next: address::Nil,
  };

  let mut m = Node {
    value: 74,
    next: address::Nil,
  };

  m.next = address::address(Box::new(Node {
    value: 4267,
    next: address::Nil,
  }));

  m.push(123);

  let mut x = Node {
    value: 42,
    next: address::address(Box::new(Node {
      value: 67,
      next: address::Nil,
    })),
  };

  n.push(32);
  n.push(64);
  n.push_node(m);
  n.push(12);
  n.push_node(x);
  n.push(45);
  

  n.print();
  n.delete(64);
  n.delete(12);
  println!("------------------");
  n.print();
}

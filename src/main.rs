fn main() {
  let x = vec![
    75, 12, 564, 542, 4832, 843123, 234234, 324, 3224, 4356, 5456, 845, 75, 12, 564, 234234, 324,
    3224, 4356, 5456,
  ];

  let equal_to_x = |z: Vec<i32>| {
    println!("Is z: {:?}", z);
    if z == x {
      println!("Match Successful")
    } else {
      println!("Match Error")
    }
  };

  println!("can't use x here: {:?}", x);

  let y = vec![
    75, 12, 564, 542, 4832, 843123, 234234, 324, 3224, 4356, 5456, 845, 75, 12, 564, 234234, 324,
    3224, 4356, 5456,
  ];

  set(equal_to_x, y.clone());

  set(
    |mut m| {
      for i in 0..m.len() {
        for e in 0..i {
          if m[e] > m[i] {
            let t = m[i];
            m[i] = m[e];
            m[e] = t;
          }
        }
      }
      println!("Order {:?}", m);
    },
    y.clone(),
  );

  set(order_dec, y);

  //set(, pv: Vec<i32>)
}

fn set<T: Fn(Vec<i32>)>(f: T, pv: Vec<i32>) {
  f(pv);
}

fn order_dec(mut m: Vec<i32>) {
  let len = m.len();
  for i in 0..len {
    for e in 0..i {
      if m[e] > m[i] {
        let t = m[i];
        m[i] = m[e];
        m[e] = t;
      }
    }
  }
  for i in 0..len / 2 {
    let t = m[len - i - 1];
    m[len - i - 1] = m[i];
    m[i] = t;
  }
  println!("Order {:?}", m);
}

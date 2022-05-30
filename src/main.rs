use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let lc = Arc::new(Mutex::new(0));
  thread::spawn(move || {
    for i in 0..10 {
      thread::sleep(std::time::Duration::from_millis(
        (i * 1000).try_into().unwrap(),
      ));
      *lc.lock().unwrap() += 1;
      println!("Hello, World!");
      println!("{}", lc.lock().unwrap());
    }
  });
  loop {}
}

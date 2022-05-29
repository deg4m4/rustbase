use std::thread;
use std::time::Duration;

fn main() {
  for i in 1..10 {
   thread::spawn(move || {
      thread::sleep(Duration::from_millis(i * 1000));
      println!("hi number {} from the spawned thread!", i);
      /* if i == 9 {
        std::process::exit(0);
      } */
    });
  }
  loop{}
}

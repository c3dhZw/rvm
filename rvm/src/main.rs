use std::env::args;

use rvm::{load_image, run};

fn main() {
  println!("hello awa");

  let file = args().nth(1).unwrap();

  load_image(&file, 0);
  run(0);
}

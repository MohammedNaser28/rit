use std::env;
use rit::{cat_file::{pretty_print, cat_file}, init::init};

fn main() {
  let args: Vec<String> = env::args().collect();
  match args[1].as_str() {
    "init" => init(),
    "cat-file" => cat_file(&args[2..]),

  _ => println!("unkown command {}",args[1]),
  }
}

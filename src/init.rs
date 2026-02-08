use std::fs;
use std::io::Read;
pub fn init(){
  fs::create_dir(".git").unwrap();
  fs::create_dir(".git/objects").unwrap();
  fs::create_dir(".git/refs").unwrap();
  fs::write(".git/HEAD","ref: refs/heads/master\n").unwrap();
  println!("Done Initaliazed Repository");
}

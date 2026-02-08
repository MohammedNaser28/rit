use std::fs;
use std::env;
use flate2::read::ZlibDecoder;
use std::io::Read;
fn main() {
  let args: Vec<String> = env::args().collect();
  if args[1] == "init" {
  fs::create_dir(".git").unwrap();
  fs::create_dir(".git/objects").unwrap();
  fs::create_dir(".git/refs").unwrap();
  fs::write(".git/HEAD","ref: refs/heads/master\n").unwrap();
  println!("Done Initaliazed Repository");
  }else if args[1] == "cat-file" {
        if args[2] == "-p" {
    let hash: String = args[3].clone();
    let folder_name: &str= &hash[0..2];
    let file_name: &str= &hash[2..];
    let path: String = format!(".git/objects/{folder_name}/{file_name}");
    let mut object = fs::File::open(&path).unwrap();
    let mut content: Vec<u8> = vec![];
        let mut extracted_content: String = String::new();
        object.read_to_end(&mut content).unwrap();
        let mut decoder = ZlibDecoder::new(content.as_slice());
decoder.read_to_string(&mut extracted_content).unwrap();
print!("{extracted_content}");


}else {
println!("unkown flag {}", args[2]);
}
  }
    else {
  println!("unkown command {}",args[1]);
  }
}

use flate2::read::ZlibDecoder;
use std::io::Read;
use std::fs;
use std::io::{self, Write};



pub fn cat_file(args: &[String]){
    if args[0].as_str() == "-p" {
        pretty_print(args[1].as_str());
    }
}

pub fn pretty_print(hash: &str){
   let folder_name: &str= &hash[0..2];
    let file_name: &str= &hash[2..];
    let path: String = format!(".git/objects/{folder_name}/{file_name}");
    let mut object = fs::File::open(&path).unwrap();
    let mut content: Vec<u8> = vec![];
        let mut extracted_content: String = String::new();
        object.read_to_end(&mut content).unwrap();
        let mut decoder = ZlibDecoder::new(content.as_slice());
decoder.read_to_string(&mut extracted_content).unwrap();
let split = extracted_content.split("\x00");
let extracted_content = split.last().unwrap();
print!("{extracted_content}");
io::stdout().flush();
}

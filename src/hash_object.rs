use flate2::write::ZlibEncoder;
use flate2::Compression;
use hex_literal::hex;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::io::{self, Read};
use std::ptr::hash;

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => write_hash_in_file(&args[1]),
        "--stdin" => {
            let mut buffer = Vec::new();
            io::stdin().lock().read_to_end(&mut buffer).unwrap();
            println!("{}", hash_content(&buffer));
        }
        _ => print!("Unkown flag"),
    }
}

fn hash_content(content: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    let header = format!("blob {}\0", content.len());
    hasher.update(header.as_bytes());
    hasher.update(content);
    let res = hasher.finalize();
    res.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

fn write_hash_in_file(path: &str) {
    // let mut hasher = Sha1::new();
    let text = fs::read(path).expect("Should have been able to read the file");
    let hash_array = hash_content(&text);
    let folder_name: &str = &hash_array[0..2];
    let file_name: &str = &hash_array[2..];
    let path_folder = format!(".git/objects/{folder_name}");
    let file_path = format!("{path_folder}/{file_name}");
    _ = fs::create_dir_all(&path_folder).unwrap();
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&text).unwrap();
    let compressed_data = e.finish().unwrap();
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(&compressed_data).unwrap();
}

fn get_type() {
    todo!();
}

use rit::{cat_file::cat_file, hash_object::hash_object, init::init};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check if we have at least one argument (the command)
    if args.len() < 2 {
        eprintln!("Usage: rit <command> [args]");
        std::process::exit(1);
    }
    match args[1].as_str() {
        "init" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            init(path)
        }
        "cat-file" => cat_file(&args[2..]),
        "hash-object" => hash_object(&args[2..]),
        _ => println!("unkown command {}", args[1]),
    }
}

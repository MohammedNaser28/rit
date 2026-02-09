use std::fs;
pub fn init(args: &str) {
    fs::create_dir_all(format!("{}/.git", args)).unwrap();
    fs::create_dir_all(format!("{}/.git/objects", args)).unwrap();
    fs::create_dir_all(format!("{}/.git/refs", args)).unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Done Initaliazed Repository");
}

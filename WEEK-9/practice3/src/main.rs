
use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("counld not remove file");
    println!("File removed successfully");
}

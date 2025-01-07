use std::fs::OpenOptions;
use std::io::Write;



fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "Cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("Write failed");
    file.write_all("\nThis is the appendage of the document.".as_bytes()).expect(
        "wite failed");
    println!("File append success");

}

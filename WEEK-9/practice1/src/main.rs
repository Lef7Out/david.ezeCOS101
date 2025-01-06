use std::io::Write;

fn main() {
    let announce = "Week 9 = Rust file Input @Output\n";
    let dept = "Department of computer science";

    let mut file = std::fs::File::create("data.txt").expect("Unable to create file");
    file.write_all("Welcome to Rust Programming\n"
       .as_bytes()).expect("Unable to write to file");
    file.write_all(announce.as_bytes()).expect("Unable to write to file");
    file.write_all(dept.as_bytes()).expect("Unable to write to file");
    println!("\nFile created");
}

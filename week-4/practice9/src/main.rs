fn main() {
    let mut count = 0;
    
    for num in 1..21 {
        if num > 10 {
            println!("{:?}", num);
            continue;
        }
        count += 1;
    }
    println!(" The count of values greater than 10 (betwwen 1 an 10) is: {}", count);
    // outputs 10
}

fn main() {
    //Write out the parameters for each variable
    Let p:f64 = 520000000;
    Let r:f64 = 10%;
    Let y:f64 = 5;

    Let a = p * (1.0 + (r/100)^ y);
    println!("Amount is {}", a);
    Let ci = a - p
    println!("Compound Intrest is {}", ci);


}

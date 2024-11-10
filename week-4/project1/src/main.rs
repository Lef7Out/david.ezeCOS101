use std::io;

fn main() {
    println!("This program is to help solve quadratcic problems using the quadratic formula");


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter your value of a:"); 
    io::stdin().read_line(&mut input1).expect("Invalid input!");
    let a:f32 = input1.trim().parse().expect("Invalid number!");

    println!("Enter your value of b:"); 
    io::stdin().read_line(&mut input2).expect("Invalid input!");
    let b:f32 = input2.trim().parse().expect("Invalid number!");


    println!("Enter your value of c:");
    io::stdin().read_line(&mut input3).expect("Invalid input!");
    let c:f32 = input3.trim().parse().expect("Invalid number!");


    let  s:f32 = b.powf(2.0) - 4.0 * a * c;
    let  y:f32 = s.sqrt();
    let quadratic1:f32 = (b-y)/ 2.0;
    let quadratic2:f32 = (b+y)/ 2.0;

    println!("The quadratic equation is: {}x^2 + {}x + {} = 0", a, b, c);
    println!("The square root of the quadratic equation is: {} and {}", quadratic1, quadratic2);
    
}

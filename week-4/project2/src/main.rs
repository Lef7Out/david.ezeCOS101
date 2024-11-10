use std::io;

fn main() {
   
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What is your age?");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let age:i32 = input1.trim().parse().expect("not a valid number");

    println!("1=Yes, 2=no");
    println!("Are you an experienced worker? (1/2)");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let experience:i32 = input2.trim().parse().expect("not a valid number");

    if age >= 40 && experience == 1{

        println!(" Your incentive is ₦1,560,000");
    }
    else if age >= 30 && age <= 40 && experience == 1{

        println!(" Your incentive is ₦1,480,000");
    }
    else if age <=29 && experience == 1{

        println!(" Your incentive is ₦1,300,000");
    }
    else {
        println!(" Your incentive is ₦100,000");
    }
    }





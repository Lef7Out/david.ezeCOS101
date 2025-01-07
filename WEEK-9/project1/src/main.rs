use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let mut lager = Vec::new();
    let mut stout = Vec::new();
    let mut non_alcoholic = Vec::new();

    println!("Enter drinks for each category. Type 'done' when finished.\n");
    
    println!("Enter Lager drinks:");
    collect_drinks(&mut lager);
  
    println!("\nEnter Stout drinks:");
    collect_drinks(&mut stout);

    println!("\nEnter Non-Alcoholic drinks:");
    collect_drinks(&mut non_alcoholic);
    
    println!("\nEnter the file name to save the drinks to:");
    let file_name = get_user_input();

    let mut file = File::create(&file_name)?;
    writeln!(file, "Lager, Stout, Non- Alcholic")?;

    let max_rows = lager.len().max(stout.len()).max(non_alcoholic.len());

    for i in 0..max_rows {
        let  lager1 = lager.get(i).unwrap_or(&String::from("")).clone();
        let  stout2 = stout.get(i).unwrap_or(&String::from("")).clone();
        let  non_alcoholic3 = non_alcoholic.get(i).unwrap_or(&String::from("")).clone();
        writeln!(file, "{}, {}, {}", lager1, stout2, non_alcoholic3)?;
    }
    println!("Drinks saved to {}", file_name);
    Ok(())
 
}

fn collect_drinks(category: &mut Vec<String>) {
    loop {
        let input = get_user_input();
        if input.eq_ignore_ascii_case("done") {
            break;
        }
        if input.trim().is_empty() {
            println!("Input cannot be empty. Please try again.");
            continue;
        }
        category.push(input);
    }
}


fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input.trim().to_string()

}

// fn collect_drinks(category: &mut Vec<String>) {
//     loop {       
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("Failed to read input");
//         let input = input.trim().to_string();
       
//         if input.eq_ignore_ascii_case("done") {
//             break;
//         }
//         if input.is_empty() {
//             println!("Input cannot be empty. Please try again.");
//         } else {
//             category.push(input.to_string());
//             println!("Added: {}", input);
//         }
//     }
// }


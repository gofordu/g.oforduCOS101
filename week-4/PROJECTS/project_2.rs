use std::io;

fn main() {
    let mut input_exp = String::new();
    let mut input_age = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input_exp).expect("Failed to read input");
    let is_experienced = input_exp.trim().to_lowercase() == "yes";

    if is_experienced {
        println!("Enter age: ");
        io::stdin().read_line(&mut input_age).expect("Failed to read input");
        let age: i32 = input_age.trim().parse().expect("Not a valid number");

        if age >= 40 {
            println!("Annual Incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual Incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual Incentive: N1,300,000");
        } else {
            println!("No incentive criteria matched for this age.");
        }
    } else {
        println!("Annual Incentive: N100,000");
    }
}
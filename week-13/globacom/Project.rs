use std::fs::File;
use std::io;
use std::io::Read;

fn administrator() {
    let mut file = File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn project_manager() {
    let mut file = File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn employee() {
    let mut file = File::open("employees_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn customer() {
    let mut file = File::open("customers_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn vendor() {
    let mut file = File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    println!("Select a role:");
    println!("1. Administrator");
    println!("2. Project manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice == "1" {
        administrator();
    } else if choice == "2" {
        project_manager();
    } else if choice == "3" {
        employee();
    } else if choice == "4" {
        customer();
    } else if choice == "5" {
        vendor();
    } else {
        println!("Invalid choice");
    }
}
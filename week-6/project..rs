use std::io;

fn main() {
    // Display the menu
    println!("========== MENU ==========");
    println!("P = Pounding Yam / Edinkaiko Soup  - N3,200");
    println!("F = Fried Rice & Chicken          - N3,000");
    println!("A = Amala & Ewedu Soup            - N2,500");
    println!("E = Eba & Egusi Soup              - N2,000");
    println!("W = White Rice & Stew             - N2,500");
    println!("==========================");

    // Read food type selection
    println!("\nEnter the food code (P, F, A, E, W):");
    let mut food_code = String::new();
    io::stdin()
        .read_line(&mut food_code)
        .expect("Failed to read line");
    let food_code = food_code.trim().to_uppercase();

    // Determine price based on selection
    let price: f64 = if food_code == "P" {
        3200.0
    } else if food_code == "F" {
        3000.0
    } else if food_code == "A" {
        2500.0
    } else if food_code == "E" {
        2000.0
    } else if food_code == "W" {
        2500.0
    } else {
        println!("Invalid food selection.");
        return;
    };

    // Read quantity
    println!("Enter quantity:");
    let mut quantity_str = String::new();
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read line");

    let quantity: f64 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for quantity.");
            return;
        }
    };

    // Calculate initial total
    let total = price * quantity;
    println!("\nSubtotal: N{:.2}", total);

    // Apply 5% discount if total exceeds N10,000
    let final_total = if total > 10000.0 {
        let discount = total * 0.05;
        println!("Discount applied (5%): -N{:.2}", discount);
        total - discount
    } else {
        total
    };

    println!("Total Amount Due: N{:.2}", final_total);
}
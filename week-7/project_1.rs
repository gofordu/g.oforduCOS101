use std::io;


// Trapezium
fn trapezium() -> f64 {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height: f64 = input1.trim().parse().expect("Invalid input");

    println!("Enter base 1:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1: f64 = input2.trim().parse().expect("Invalid input");

    println!("Enter base 2:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2: f64 = input3.trim().parse().expect("Invalid input");

    // Return the calculated area
    (height / 2.0) * (base1 + base2)
}

// Rhombus
fn rhombus() -> f64 {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let d1: f64 = input1.trim().parse().expect("Invalid input");

    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let d2: f64 = input2.trim().parse().expect("Invalid input");

    0.5 * d1 * d2
}

// Parallelogram
fn parallelogram() -> f64 {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Invalid input");

    println!("Enter altitude:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Invalid input");

    base * altitude
}

// Cube
fn cube() -> f64 {
    let mut input = String::new();

    println!("Enter side:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let side: f64 = input.trim().parse().expect("Invalid input");

    6.0 * side * side
}

// Cylinder
fn cylinder() -> f64 {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Invalid input");

    println!("Enter height:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Invalid input");

    3.1416 * radius * radius * height
}

fn main() {
    let shapes: [&str; 5] = [
        "Trapezium", 
        "Rhombus", 
        "Parallelogram", 
        "Cube", 
        "Cylinder"
    ];

    println!("SHAPE CALCULATOR");
    println!("Choose a shape:");

    for i in 0..shapes.len() {
        println!("{}", shapes[i]);
    }

    let mut choice_input = String::new();
    println!("Enter choice (1-5):");
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice: i32 = choice_input.trim().parse().expect("Invalid input");

    let result = if choice == 1 {
        trapezium()
    } else if choice == 2 {
        rhombus()
    } else if choice == 3 {
        parallelogram()
    } else if choice == 4 {
        cube()
    } else if choice == 5 {
        cylinder()
    } else {
        println!("Invalid choice selected!");
        return;
    };

    println!("The final answer is: {}", result);
}
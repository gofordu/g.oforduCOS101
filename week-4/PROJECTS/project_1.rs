use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a: f32 = input_a.trim().parse().expect("Not a valid number");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b: f32 = input_b.trim().parse().expect("Not a valid number");

    println!("Enter value for c: ");
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c: f32 = input_c.trim().parse().expect("Not a valid number");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Exactly one real root: {}", root);
    } else {
        println!("No real roots");
    }
}
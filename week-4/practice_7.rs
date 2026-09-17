use std::io;[cite: 6]

fn main() {[cite: 6]

    println!("Enter a number");[cite: 6]
    let mut input1 = String::new();[cite: 6]
    io::stdin().read_line(&mut input1).expect("Failed to read input");[cite: 6]
    let mut num:i32 = input1.trim().parse().expect("Failed to input");[cite: 6]

    while num < 10 {[cite: 6]

        println!("inside loop number value is {}",num);[cite: 6]
        num+=1;[cite: 6]
    }[cite: 6]
    println!("outside loop number value is {}",num);[cite: 6]
}[cite: 6]
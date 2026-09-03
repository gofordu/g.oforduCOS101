
fn main() {
    let t: f64 = 450000.00; // Toshiba
    let m: f64 = 1500000.00; // Mac
    let h: f64 = 750000.00; // HP
    let d: f64 = 2850000.00; // Dell
    let a: f64 = 250000.00; // Acer

    // Sum
    let sum: f64 = t + m + h + d + a;

    let count = [t, m, h, d, a].len() as f64;

    // Calculate average
    let average = sum / count;

    println!("P.M. Okeke and Sons Ltd - Sales Summary");
    println!(" ");
    println!("Total Sales Amount: ₦{}", sum);
    println!("Average Sales Amount: ₦{}", average);
}
fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	//depreciation
	let a = p * (1.0 - (r / 100.0)).powf(t);
	println!("TV depriciation");
	println!(" ");
	println!("Amount is {}", a);
	let d = p-a;
	println!("The depriciation of Ms. Akudo Ijizie's TV is {}", d );
}
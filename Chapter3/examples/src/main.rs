fn main() {
	let x: u8 = 5;
    println!("The value of x is: {x}");
	let x = x + 1;
	
 	{
		let x = x * 2;
		println!("Inner scope x is: {x}");
	}
	println!("The value of x is: {x}");
}
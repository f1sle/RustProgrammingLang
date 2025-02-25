fn main() {
	let tup = (500, 6.4, 1);
	let (x, y, z) = tup;
    // this is called destructuring
	println!("Values are: x = {x}, y = {y}, z = {z}");

    let t: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = t.0;
    let six_point_four = t.1;
    let one = t.2;
}
fn main() {
    let number = 3u8;

    if number < 5 {
        println!("condition was true");
    } else if number % 3 == 0 { // else if example
        println!("number is divisible by 3");
    } else {
        println!("condition was false");
    }

    // unfortunately, code below won't work in Rust because Rust is type strict
    // if number {
    //     println!("number was there");
    // }
    // if will result in an error
    //      if number {
    //         ^^^^^^ expected `bool`, found integer
    
    // we should use instead:
    if number != 0 {
        println!("{number} is greater than 0 !");
    }

    // IF in a LET statement example
    let condition = true;

    let num = if condition { 5 } else { 6 };

    println!("The value of number is: {num}");

    // although code below won't work
    // let n = if condition { 5 } else { "six" };
    // will result in an error because types should be the same
}
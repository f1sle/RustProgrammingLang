fn main() {
    let x = 6; // statement

    // you can't do this in Rust: let x = (let y = 6);
    // error: variable declaration using `let` is a statement
    println!("The value of x is: {x}");
    
    // statement which is using expressions to assign itself a value
    let y = {
        let x = 3; 
        x + 1  // expressions don't require a semicolon;otherwise, it would be a statement
    };
    println!("The value of y is: {y}");
}
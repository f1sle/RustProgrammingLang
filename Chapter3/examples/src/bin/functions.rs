fn main() {
    println!("Hello, world!");

    another_function();
    param_function(111);
    multiple_param_function(10, 'T');
}

fn another_function() {
    println!("Another function.");
}

// function with parameters
// In function signatures, you must declare the type of each parameter.
fn param_function(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_param_function(value:i32, unit_label:char) {
    println!("The measurement is: {value} {unit_label}");
}
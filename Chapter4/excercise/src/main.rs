fn main() {
    // Ownership and Functions
    let s = String::from("hello");

    takes_ownership(s);
    // s no longer valid down the code
    // println!("{s}"); will raise an error
    let x:i32 = 5;
    makes_copy(x);

    println!("X is still valid and we can see its value: {x}");
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
    // Here, some_string goes out of scope and `drop` is called.
    // The backing memory is freed. 
}

fn makes_copy(some_int: i32) { // some_int comes into scope
    println!("{some_int}");
    // Here, some_int goes out of scope. Nothing special happens.
}

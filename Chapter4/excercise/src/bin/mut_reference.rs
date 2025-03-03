fn main() {
    let mut s = String::from("hello");
    println!("s is: {s}");
    change(&mut s);
    println!("s is: {s}");

    // data race conditions:
    // two or more pointers access the same data at the same time
    // at least one of the pointers is being used to write to the data
    // there's no mechanism being used to synchronize access to the data
    let mut s1 = String::from("hello");
    {
        let r1 = &mut s1;
        // r1 goes out of scope here, so we can make a new ref with no problems
    }

    let r2 = &mut s1;

    // let r1 = &s1; // no problem
    // let r2 = &s1; // no problem
    // let r3 = &mut s1; // BIG PROBLEM because it's going to modify it
    // println!("{r1} {r2} {r3}");
}

fn change(some_string: &mut String) { 
    some_string.push_str(", Batman!");
}
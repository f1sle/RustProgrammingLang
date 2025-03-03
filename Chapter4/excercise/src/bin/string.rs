fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`


    let s1 = String::from("HELLO");

    //let s2 = s1; // will raise an error because s1 is no longer valid
    //println!("{s1} {s2}");
    // we should use s1.clone(); instead
    let a1 = String::from("a1 string");
    let a2 = a1.clone();

    println!("a1 = {a1}, a2 = {a2}");
}
fn main() {
    let ref_to_nothing = dangle(); 
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("he"); // s is new string
    &s                          // we return a ref to the String s
} // Here, s goes out of scope and is dropped, so its memory goes away
// Danger!

fn no_dangle() -> String {
    let s = String::from("he");
    s // will return the string itself so no error 
}
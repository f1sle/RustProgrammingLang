fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("First word of string '{s}' is: {word}");

    s.clear(); // error! if we put println after this line there would be ownership error
    // we can not have immutable & mutable refs at the same time
    // this empties the String, making it equal to ""
}



fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
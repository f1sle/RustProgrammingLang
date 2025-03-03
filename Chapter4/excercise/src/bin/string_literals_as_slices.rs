fn main() {
    let my_string = String::from("Hello world");

    // first_word works on slices of 'Strinh's, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // first_word also works on references to 'String's, which are
    // equivalent to whole slices's of 'String's.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    
    // first_word works on slices of string literals,
    // whether partial or whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}





let s = "Hello, World!"; // type of s here is &str
// which is immutable reference

// more experiences Rustacean would write the first_word function like this
fn first_word(s: &str) -> &str { // it would allow to use both: &str and &String
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
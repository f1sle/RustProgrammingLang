fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s2 = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]; // if you start from 0 then you can omit it

    let len = s.len();

    let slice2 = &s[3..len];
    let slice2 = &s[3..]; // same works with trailing indexes as well
    let slice3 = &s[..]; // we can also drop the both indexes to get a slice
                         // of the entire string ASCII only
}
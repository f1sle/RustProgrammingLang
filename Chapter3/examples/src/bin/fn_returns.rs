fn main() {
    println!("Five is: {}", five());
}

fn five() -> u8 {
    5 // semicolon here result in an error because it'll transform it to a statement, which don't return a value
}
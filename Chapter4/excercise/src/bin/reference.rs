fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // opposite to reference is '*' dereference operator

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, the String is not dropped.
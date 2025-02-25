fn main() {
    let mut number = 3u8;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!!");

    let a = [10, 20, 30, 40, 50]; // fixed array with 5 elements
    let mut index = 0;

    while index < 5 { // if we will change array a to contain other number of elements other than 5 there would be an error
        print!("{} ", a[index]);
        index += 1;
    }
    println!("");

    // error prone looping through the array
    for element in a {
        println!("the value is: {element}");
    }
    println!("");
    // loop through array elements in a reversed order
    for number in (1..=10).rev() {
        println!("{number}");
    }
}
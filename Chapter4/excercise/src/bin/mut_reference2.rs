fn main() {
    let mut s = String::from("happy");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{r1} {r2}");
    // if variables r1 and r2 won't be used later then no problem
    let r3 = &mut s;
    println!("{r3}");
    // println!("{r1} {r2}"); will raise an error - not good
}
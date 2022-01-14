#![allow(unused_variables, unused_assignments)]
pub fn temp() -> i32 {
    return 1;
}

fn main() {
    let _x = &temp();
    // temp() = *x;     // E0070


    let a = 1;
    // a = 2;   // immutable and error
    let mut b = 2;
    b = 3;


    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", other);
    let other = place2;
    println!("{:?}", other);
}

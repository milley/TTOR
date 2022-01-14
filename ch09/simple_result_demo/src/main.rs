use std::num::ParseIntError;

// fn square(number_str: &str) -> Result<i32, ParseIntError> {
//     number_str.parse::<i32>().map(|n| n.pow(2))
// }

type ParseResult<T> = Result<T, ParseIntError>;
fn square(number_str: &str) -> ParseResult<i32> {
    number_str.parse::<i32>().map(|n| n.pow(2))
}


use std::panic;
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    match square("10") {
        Ok(n) => assert_eq!(n, 100),
        Err(err) => println!("Error: {:?}", err),
    }


    // let result = panic::catch_unwind(|| { println!("hello!"); });
    // assert!(result.is_ok());
    // let result = panic::catch_unwind(|| { panic!("oh no!"); });
    // assert!(result.is_err());
    // println!("{}", sum(1, 2));
    let result = panic::catch_unwind(|| { println!("hello!"); });
    assert!(result.is_ok());
    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            println!("panic occurred '{}' at {}", location.file(), location.line());
        } else {
            println!("can't get location information...");
        }
    }));
    let result = panic::catch_unwind(|| { panic!("oh no!"); });
    assert!(result.is_err());
    println!("{}", sum(1, 2));
}

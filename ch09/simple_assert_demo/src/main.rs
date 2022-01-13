fn get_shortest(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
        let mut shortest = names[0];
        for name in names.iter() {
            if name.len() < shortest.len() {
                shortest = *name;
            }
        }
        Some(shortest)
    } else {
        None
    }
}
// fn show_shortest(names: Vec<&str>) -> &str {
//     match get_shortest(names) {
//         Some(shortest) => shortest,
//         None => "Not Found",
//     }
// }
fn show_shortest(names: Vec<&str>) -> &str {
    // get_shortest(names).unwrap()
    get_shortest(names).unwrap_or("Not Found")
    // get_shortest(names).unwrap_or_else(||"Not Found")
    // get_shortest(names).expect("Not Found")
}
// fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
//     match get_shortest(names) {
//         Some(shortest) => Some(shortest.len()),
//         None => None,
//     }
// }

fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
    get_shortest(names).map(|name| name.len())
}



fn double(value: f64) -> f64 { value * 2. }
fn square(value: f64) -> f64 { value.powi(2 as i32) }
fn inverse(value: f64) -> f64 { value * -1. }
fn log(value: f64) -> Option<f64> {
    match value.log2() {
        x if x.is_normal() => Some(x),
        _ => None,
    }
}
fn sqrt(value: f64) -> Option<f64> {
    match value.sqrt() {
        x if x.is_normal() => Some(x),
        _ => None,
    }
}

fn main() {
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);
    // vec.insert(8, 8);


    // let x = false;
    // assert!(x, "x wasn't true!");
    // let a = 3;
    // let b = 28;
    // debug_assert!(a + b == 30, "a = {} and b = {}", a, b);



    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
    assert_eq!(get_shortest_length(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);



    let number: f64 = 20.;
    let result = Option::from(number)
        .map(inverse).map(double).map(inverse)
        .and_then(log).map(square).and_then(sqrt);
    match result {
        Some(x) => println!("Result was {}.", x),
        None => println!("This is failed."),
    }


    let n = "1";
    assert_eq!(n.parse::<i32>(), Ok(1));
    let n = "a";
    println!("{:?}", n.parse::<i32>());
}

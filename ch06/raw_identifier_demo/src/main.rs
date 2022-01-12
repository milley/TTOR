fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

// fn modify(mut v: Vec<u32>) -> Vec<u32> {
//     v.push(42);
//     v
// }
fn modify(v: &mut [u32]) {
    v.reverse();
}


fn f() { println!("1"); }


#[derive(Debug)]
struct S { i: i32 }
fn f1(ref _s: S) {
    println!("{:?}", _s);
}


fn swap((x, y): (&str, i32)) -> (i32, &str) {
    (y, x)
}


fn addsub(x: isize, y: isize) -> (isize, isize) {
    (x + y, x - y)
}


fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { return a; }
    return gcd(b, a % b);
}


use std::ops::Mul;
fn square<T: Mul<T, Output=T>>(x: T, y: T) -> T {
    x * y
}


#[derive(Debug)]
struct User {
    name: &'static str,
    avatar_url: &'static str,
}
impl User {
    fn show(&self) {
        println!("name: {:?}", self.name);
        println!("avatar_url: {:?}", self.avatar_url);
    }
}


// fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
//     op(a, b)
// }

// type MathOp = fn(i32, i32) -> i32;
// fn math(op: MathOp, a: i32, b: i32) -> i32 {
//     op(a, b)
// }

// fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn product(a: i32, b: i32) -> i32 {
//     a * b
// }


type MathOp = fn(i32, i32) -> i32;
fn math(op: &str) -> MathOp {
    fn sum(a: i32, b: i32) -> i32 { a + b }
    fn product(a: i32, b: i32) -> i32 { a * b }
    match op {
        "sum" => sum,
        "product" => product,
        _ => {
            println!("Warning: Not implemented {:?} operator, Replace with sum", op);
            sum
        }
    }
}

// Compile error: expected fn pointer, found `i32`
// fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn product(a: i32, b: i32) -> i32 {
//     a * b
// }

// type MathOp = fn(i32, i32) -> i32;
// fn math(op: &str, a: i32, b: i32) -> MathOp {
//     match op {
//         "sum" => sum(a, b),
//         _ => product(a, b),
//     }
// }



fn hello() { println!("hello function pointer"); }


// fn counter() -> fn(i32) -> i32 {
//     fn inc(n: i32) -> i32 {
//         n + 1
//     }
//     inc
// }


// Compile error: can't capture dynamic environment variable
// fn counter(i: i32) -> fn(i32) -> i32 {
//     fn inc(n: i32) -> i32 {
//         n + i
//     }
//     inc
// }


fn counter(i: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |n: i32| n + i)
}

fn val() -> i32 { 5 }


fn main() {
    assert!(r#match("foo", "foobar"));


    // let v = vec![1, 2, 3];
    // let v = modify(v);
    // println!("{:?}", v);

    let mut v  = vec![1, 2, 3];
    modify(&mut v);
    println!("{:?}", v);


    f();
    {
        f();
        fn f() { println!("3"); }
    }
    f();
    fn f() { println!("2"); }


    let s = S{ i: 42 };
    f1(s);


    let t = ("Alex", 18);
    let t = swap(t);
    assert_eq!(t, (18, "Alex"));


    let (a, b) = addsub(5, 8);
    println!("a: {:?}, b: {:?}", a, b);


    let g = gcd(60, 40);
    assert_eq!(20, g);


    // let a: i32 = square(37, 41);
    // let b: f64 = square(37.2, 41.1);
    // assert_eq!(a, 1517);
    // assert_eq!(b, 1528.92);
    let a = square::<u32>(37, 41);
    let b = square::<f32>(37.2, 41.1);
    assert_eq!(a, 1517);
    assert_eq!(b, 1528.9199);



    let user = User {
        name: "Alex",
        avatar_url: "https://avatar.com/alex"
    };
    user.show();



    // let (a, b) = (2, 3);
    // assert_eq!(math(sum, a, b), 5);
    // assert_eq!(math(product, a, b), 6);


    let fn_ptr: fn() = hello;
    println!("{:p}", fn_ptr);
    let other_fn = hello;
    // println!("{:p}", other_fn);
    hello();
    other_fn();
    fn_ptr();
    (fn_ptr)();


    let (a, b) = (2, 3);
    let sum = math("sum");
    let product = math("product");
    let div = math("div");
    assert_eq!(sum(a, b), 5);
    assert_eq!(product(a, b), 6);
    assert_eq!(div(a, b), 5);


    // let f = counter();
    // assert_eq!(2, f(1));

    let f = counter(2);
    assert_eq!(3, f(1));


    // let add = |a: i32, b: i32| -> i32 { a + b };
    // assert_eq!(add(1, 2), 3);
    let add = |a: fn() -> i32, (b, c)| (a)() + b + c;
    let r = add(val, (2, 3));
    assert_eq!(r, 10);


    let c1 = || {};
    let c2 = || {};
    let v = [c1, c2];


    // let c1: () = || { println!("i'm a closure") };


    let s = "hello";
    let c = || { println!("{:?}", s) };
    c();
    c();
    println!("{:?}", s);


    let s = "hello".to_string();
    let c = ||s;
    c();
    // c();
    // println!("{:?}", s);
}

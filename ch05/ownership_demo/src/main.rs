#[derive(Debug, Copy, Clone)]
struct A {
    a: i32,
    b: u32,
}


fn foo(s: String) -> String {
    let w = " world".to_string();
    s + &w
}

fn main() {
    let x = 5;
    let y = x;
    assert_eq!(x, 5);
    assert_eq!(y, 5);


    let x = Box::new(5);
    let y = x;
    // println!("{:?}", x);
    println!("{:?}", y);


    let a = A { a: 1, b: 2 };
    let b = a;
    println!("{:?}", a);


    let a = ("a".to_string(), "b".to_string());
    let b = a;
    // println!("{:?}", a);
    let c = (1, 2, 3);
    let d = c;
    println!("{:?}", d);


    let outer_val = 1;
    let outer_sp = "hello".to_string();
    {
        let inner_val = 2;
        outer_val;
        outer_sp;
    }
    println!("{:?}", outer_val);
    // println!("{:?}", inner_val);
    // println!("{:?}", outer_sp);


    let a = Some("hello".to_string());
    match a {
        Some(s) => println!("{:?}", s),
        _ => println!("nothing",)
    }
    // println!("{:?}", a);


    let v = vec![1, 2, 3];
    for i in v {
        println!("{:?}", i);
        // println!("{:?}", v);
    }


    let a = Some("hello".to_string());
    if let Some(s) = a {
        println!("{:?}", s);
    }

    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }


    let s = "hello".to_string();
    let ss = foo(s);
    // println!("{:?}", s);


    let s = "hello".to_string();
    let join = |i: &str| { s + i };
    assert_eq!("hello world", join(" world"));
    // println!("{:?}", s);
}

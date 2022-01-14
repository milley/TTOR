use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::Arc;

// trait Foo {}
// struct Bar<'a> {
//     x: &'a i32,
// }
// impl<'a> Foo for Bar<'a> {}


trait Foo<'a> {}
struct FooImpl<'a> {
    s: &'a [u32],
}
impl<'a> Foo<'a> for FooImpl<'a> {}
fn foo<'a>(s: &'a [u32]) -> Box<dyn Foo<'a> + 'a> {
    Box::new(FooImpl { s: s })
}


struct Node {
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>,
}
impl Drop for Node {
    fn drop(&mut self) { println!("Dropping!"); }
}


use std::cell::Cell;
struct Foo_Cell {
    x: u32, y: Cell<u32>,
}


use std::borrow::Cow;
fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}
fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    lst.iter().fold(0, |acc, &n| acc + n)
}


use std::thread;
#[derive(Debug)]
struct Token<'a> {
    raw: Cow<'a, str>,
}
impl<'a> Token<'a> {
    pub fn new<S>(raw: S) -> Token<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Token { raw: raw.into() }
    }
}

fn foo_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() % 2 == 0 {
        x
    } else {
        y
    }
}


fn capitalize(data: &mut [char]) {
    // do something
}

fn foo_cap() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    capitalize(slice);
    data.push('d');
}


struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}
fn to_refs<T>(mut list: &mut List<T>) -> Vec<&mut T> {
    let mut result = vec![];
    loop {
        result.push(&mut list.value);
        if let Some(n) = list.next.as_mut() {
            list = n;
        } else {
            return result;
        }
    }
}



fn main() {
    // let num = 5;
    // let box_bar = Box::new(Bar { x: &num });
    // let obj = box_bar as Box<dyn Foo>;


    let x = Box::new("hello");
    let y = x;
    // println!("{:?}", x);


    let a = Box::new("hello");
    let b = Box::new("Rust".to_string());
    let c = *a;
    let d = *b;
    println!("{:?}", a);
    // println!("{:?}", b);


    let r = Rc::new("Rust".to_string());
    let a = Arc::new(vec![1.0, 2.0, 3.0]);
    // let x = *r;
    // println!("{:?}", r);
    // let f = *foo;


    let x = Rc::new(45);
    let y1 = x.clone();
    let y2 = x.clone();
    println!("{:?}", Rc::strong_count(&x));
    let w = Rc::downgrade(&x);
    println!("{:?}", Rc::weak_count(&x));
    let y3 = &*x;
    println!("{}", 100 - *x);



    let first = Rc::new(RefCell::new(Node { next: None, head: None }));
    let second = Rc::new(RefCell::new(Node { next: None, head: None }));
    let third = Rc::new(RefCell::new(Node { next: None, head: None }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(third.clone());
    third.borrow_mut().head = Some(Rc::downgrade(&first));


    let foo = Foo_Cell { x: 1, y: Cell::new(3) };
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);
    assert_eq!(5, foo.y.get());


    let x = RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}", x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}", x.borrow());


    let x = RefCell::new(vec![1, 2, 3, 4]);
    let mut mut_v = x.borrow_mut();
    mut_v.push(5);
    // let mut mut_v2 = x.borrow_mut();


    let s1 = [1, 2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("IN {:?}", s1);
    println!("OUT {:?}", i1);
    let s2 = [1, 2, 3, -45, 5];
    let mut i2 = Cow::from(&s2[..]);
    abs_all(&mut i2);
    println!("IN {:?}", s2);
    println!("OUT {:?}", i2);
    let mut v1 = Cow::from(vec![1, 2, -3, 4]);
    abs_all(&mut v1);
    println!("IN/OUT: {:?}", v1);
    let s3 = [1, 3, 5, 6];
    let sum1 = abs_sum(&s2[..]);
    println!("{:?}", s3);
    println!("{:?}", sum1);
    let s4 = [1, -3, 5, -6];
    let sum2 = abs_sum(&s4[..]);
    println!("{:?}", s4);
    println!("{}", sum2);



    let token = Token::new("abc123");
    let token = Token::new("api.example.io".to_string());
    thread::spawn(move || {
        println!("token: {:?}", token);
    }).join().unwrap();
    
    // let raw = String::from("abc");
    // let s = &raw[..];
    // let token = Token::new(s);
    // thread::spawn(move || {
    //     println!("token: {:?}", token);
    // }).join().unwrap();


    // Compiler error: borrowed value does not live long enough
    // let mut data = vec![1, 2, 3, 4];
    // for i in 0..4 {
    //     thread::spawn(move || {
    //         data[i] += 1;
    //     });
    //     thread::sleep_ms(50);
    // }

    let x = String::from("hello");
    let z;
    let y = String::from("world");
    z = foo_str(&x, &y);
    println!("{:?}", z);


    // let mut x = vec![1];
    // x.push(x.pop().unwrap());
}

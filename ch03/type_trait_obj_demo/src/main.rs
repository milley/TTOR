use std::thread;
use std::rc::Rc;


#[derive(Debug)]
struct Foo;
trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) { println!("{:?}", self) }
}

fn static_dispatch<T>(t: &T)
where
    T: Bar
{
    t.baz();
}

fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}


use std::fmt::Debug;
pub trait Fly {
    fn fly(&self) -> bool;
}

#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;
impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}
impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }
}
fn fly_static(s: impl Fly + Debug) -> bool {
    s.fly()
}
fn can_fly(s: impl Fly + Debug) -> impl Fly {
    if s.fly() {
        println!("{:?} can fly", s);
    } else {
        println!("{:?} can't fly", s);
    }
    s
}


// use std::ops::Add;
// fn sum<T>(a: impl Add<Output=T>, b: impl Add<Output=T>) -> T {
//     a + b
// }


fn test_copy<T: Copy>(i: T) {
    println!("hhh");
}


fn foo_i32(s: &[i32]) {
    println!("{:?}", s[0]);
}


struct S(i32);
trait A {
    fn test(&self, i: i32);
}
trait B {
    fn test(&self, i: i32);
}
impl A for S {
    fn test(&self, i: i32) {
        println!("From A: {:?}", i);
    }
}
impl B for S {
    fn test(&self, i: i32) {
        println!("From B: {:?}", i);
    }
}


#[derive(Debug)]
struct Person { name: String }
impl Person {
    fn new<T: Into<String>>(name: T) -> Self {
        Person { name: name.into() }
    }
}


use std::ops::Add;
#[derive(PartialEq)]
struct Int(i32);
impl Add<i32> for Int {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        (self.0) + other
    }
}

impl Add<i32> for Box<Int> {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        (self.0) + other
    }
}


// Compile as Nightly version:

// #![feature(specialization)]
// struct Diver<T> {
//     inner: T,
// }
// trait Swimmer {
//     fn swim(&self) { println!("swimming") }
// }
// impl<T> Swimmer for Diver<T> {
//     default fn swim(&self) { println!("gogogo") }
// }
// impl Swimmer for Diver<&'static str> {
//     fn swim(&self) { println!("drowning, help!") }
// }

fn main() {
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);


    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);
    let pig = Pig;
    let pig = can_fly(pig);
    let duck = Duck;
    let duck = can_fly(duck);


    let a = "String".to_string();
    // test_copy(a);


    // let x = vec![1, 2, 3, 4, 5];
    // thread::spawn(|| x);

    // let mut x = vec![1, 2, 3, 4, 5];
    // thread::spawn(|| {
    //     x.push(1);
    // });
    // x.push(2);

    // let mut x = vec![1, 2, 3, 4, 5];
    // thread::spawn(move || {
    //     x.push(1);
    // });
    // // x.push(2);

    // let x = Rc::new(vec![1, 2, 3, 4, 5]);
    // thread::spawn(move || {
    //     x[1];
    // });


    let a = "hello".to_string();
    let b = " world".to_string();
    let c = a + &b;
    println!("{:?}", c);


    let v = vec![1, 2, 3];
    foo_i32(&v);


    let x = Rc::new("hello");
    println!("{:?}", x.chars());


    let x = Rc::new("hello");
    let y = x.clone();   // Rc<&str>
    let z = (*x).clone();   // &str


    let x = "hello".to_string();
    //match x.as_ref() {
    //use std::ops::Deref;
    //match x.deref() {
    // use std::borrow::Borrow;
    // match x.borrow() {
    // match &*x {
    match &x[..] {
        "hello" => println!("hello"),
        _ => {}
    }


    let a = std::u32::MAX;
    let b = a as u16;
    assert_eq!(b, 65535);
    let e = -1i32;
    let f = e as u32;
    println!("{:?}", e.abs());
    println!("{:?}", f);


    let s = S(1);
    A::test(&s, 1);
    B::test(&s, 1);
    <S as A>::test(&s, 1);
    <S as B>::test(&s, 1);


    let a: &'static str = "hello";
    let b: &str = a as &str;
    let c: &'static str = b as &'static str;


    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);


    let person = Person::new("Alex");
    let person = Person::new("Alex".to_string());
    println!("{:?}", person);


    let a = "hello";
    let b: String = a.into();


    assert_eq!(Int(3) + 3, 6);
    assert_eq!(Box::new(Int(3)) + 3, 6);


    // let x = Diver::<&'static str> { inner: "Bob" };
    // x.swim();
    // let y = Diver::<String> { inner: String::from("Alice") };
    // y.swim();
}

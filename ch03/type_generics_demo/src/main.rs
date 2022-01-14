#[derive(Debug, PartialEq)]
struct Foo(i32);
#[derive(Debug, PartialEq)]
struct Bar(i32, i32);

trait Inst {
    fn new(i: i32) -> Self;
}

impl Inst for Foo {
    fn new(i: i32) -> Foo {
        Foo(i)
    }
}

impl Inst for Bar {
    fn new(i: i32) -> Bar {
        Bar(i, i + 10)
    }
}

fn foobar<T: Inst>(i: i32) -> T {
    T::new(i)
}


// trait Add<RHS, Output> {
//     fn my_add(self, rhs: RHS) -> Output;
// }

// impl Add<i32, i32> for i32 {
//     fn my_add(self, rhs: i32) -> i32 {
//         self + rhs
//     }
// }

// impl Add<u32, i32> for u32 {
//     fn my_add(self, rhs: u32) -> i32 {
//         (self + rhs) as i32
//     }
// }



// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }
// impl Add<u64> for u32 {
//     type Output = u64;
//     fn add(self, other: u64) -> Self::Output {
//         (self as u64) + other
//     }
// }


use std::ops::Add;
#[derive(Debug)]
struct Point {
    x: i32, y: i32,
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Point { x: self.x + other.x, y: self.y + other.y, }
    }
}


trait Page {
    fn set_page(&self, p: i32) {
        println!("Page default: 1");
    }
}

trait PerPage {
    fn set_perpage(&self, num: i32) {
        println!("Per Page default: 10");
    }
}

struct MyPaginate { page: i32 }
impl Page for MyPaginate {}
impl PerPage for MyPaginate {}

trait Paginate: Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("Skip Page: {:?}", num);
    }
}
impl<T: Page + PerPage> Paginate for T {}



fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));
    let b: Bar = foobar(20);
    assert_eq!(b, Bar(20, 30));


    // let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    // let x: i32 = a.my_add(b);
    // let y: i32 = c.my_add(d);
    // assert_eq!(x, 3i32);
    // assert_eq!(y, 7i32);


    let a = "hello";
    let b = " world";
    let c = a.to_string() + b;
    println!("{:?}", c);

    // let a = 1u32;
    // let b = 2u64;
    // assert_eq!(a.add(b), 3);


    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });


    let my_paginate = MyPaginate { page: 1 };
    my_paginate.set_page(2);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(12);


    assert_eq!(sum(1u32, 2u32), 3);
    assert_eq!(sum(1u64, 2u64), 3);
}

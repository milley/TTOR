struct A {
    a: u8,
    b: u32,
    c: u16,
}
union U {
    f1: u32,
    f2: f32,
    f3: f64,
}


use std::ops::Drop;
#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}


fn create_box() {
    let box3 = Box::new(3);
}



// type NodePtr<T> = Option<Box<Node<T>>>;
// struct Node<T> {
//     data: T,
//     next: NodePtr<T>,
// }

// use std::rc::Rc;
// type NodePtr<T> = Option<Rc<Node<T>>>;
// struct Node<T> {
//     data: T,
//     next: NodePtr<T>,
// }

use std::rc::Rc;
use std::cell::RefCell;
type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    data: T,
    next: NodePtr<T>,
}
// will not execute because it is memory safety
impl<T> Drop for Node<T> {
    fn drop(&mut self) { println!("Dropping Node!"); }
}


// dangling pointer
// fn foo<'a>() -> &'a str {
//     let a = "hello".to_string();
//     &a
// }


struct AA {
    a: u32,
    b: Box<u64>,
}
struct BB(i32, f64, char);
struct NN;
enum EE {
    H(u32),
    M(Box<u32>),
}
union UU {
    u: u32,
    v: u64,
}


fn main() {
    println!("{:?}", std::mem::size_of::<A>());
    println!("{:?}", std::mem::size_of::<U>());


    let x: i32;
    if true {
        x = 1;
    } else {
        x = 2;
    }
    println!("{:?}", x);


    let x: i32;
    loop {
        if true {
            x = 2;
            break;
        }
    }
    println!("{:?}", x);


    let a: Vec<i32> = vec![];
    let b: [i32; 0] = [];


    let x = 42;
    let y = Box::new(5);
    println!("{:p}", y);
    let x2 = x;
    let y2 = y;
    // println!("{:p}", y);


    let s = String::from("hello");
    // let deref_s: str = *s;
    let v = vec![1, 2, 3, 4];
    // let deref_v: [u32] = *v;


    let x = S(1);
    println!("crate x: {:?}", x);
    {
        let y = S(2);
        println!("crate y: {:?}", y);
        println!("exit inner scope");
    }
    println!("exit main");


    let box1 = Box::new(1);
    {
        let box2 = Box::new(2);
    }
    for _ in 0..1_000 {
        create_box();
    }


    let mut v = vec![1, 2, 3, 4];
    {
        v
    };
    // v.push(5);


    let x = S(1);
    println!("crate x: {:?}", x);
    let x = S(2);
    println!("create shadowing x: {:?}", x);


    // let mut first = Box::new(Node { data: 1, next: None} );
    // let mut second = Box::new(Node { data: 2, next: None} );
    // first.next = Some(second);
    // second.next = Some(first);

    // let mut first = Rc::new(Node { data: 1, next: None} );
    // let mut second = Rc::new(Node { data: 2, next: Some(first.clone())} );
    // first.next = Some(second.clone());
    // second.next = Some(first.clone());

    let mut first = Rc::new(RefCell::new(Node { data: 1, next: None, } ));
    let mut second = Rc::new(RefCell::new(Node { data: 2, next: Some(first.clone()), } ));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());


    // let x = foo();


    println!("Box<u32>: {:?}", std::mem::size_of::<Box::<u32>>());
    println!("AA: {:?}", std::mem::size_of::<AA>());
    println!("BB: {:?}", std::mem::size_of::<BB>());
    println!("NN: {:?}", std::mem::size_of::<NN>());
    println!("EE: {:?}", std::mem::size_of::<EE>());
    println!("UU: {:?}", std::mem::size_of::<UU>());
}

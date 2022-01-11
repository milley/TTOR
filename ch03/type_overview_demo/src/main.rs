// fn reset(mut arr: [u32; 5]) {
//     arr[0] = 5;
//     arr[1] = 4;
//     arr[2] = 3;
//     arr[3] = 2;
//     arr[4] = 1;

//     println!("reset arr {:?}", arr);
// }

fn reset(arr: &mut [u32]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;

    println!("array length {:?}", arr.len());
    println!("reset arr {:?}", arr);
}


enum Void{}
struct Foo;
struct Baz {
    foo: Foo,
    qux: (),
    baz: [u8; 0],
}


// fn foo() -> ! {
//     loop {
//         println!("jh");
//     }
// }


fn sum(a: u32, b: i32) -> u32 {
    a + (b as u32)
}


fn foo<T>(x: T) -> T {
    return x;
}


#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point {x, y}
    }
}

fn main() {
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}", ptr);
    println!("{:?}", len);



    // let arr: [u32; 5] = [1, 2, 3, 4, 5];
    // reset(arr);
    // println!("origin arr {:?}", arr);

    let mut arr = [1, 2, 3, 4, 5];
    println!("reset before: origin array {:?}", arr);
    {
        let mut_arr: &mut [u32] = &mut arr;
        reset(mut_arr);
    }
    
    println!("origin arr {:?}", arr);


    assert_eq!(std::mem::size_of::<&[u32; 5]>(), 8);
    assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);


    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<Foo>(), 0);
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0);


    let v: Vec<()> = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }


    // let i = if false { foo(); } else { 100 };
    // assert_eq!(i, 100);


    // let res: Result<u32, Void> = Ok(0);
    // let Ok(num) = res;


    let a = 1;
    let b = 2;
    assert_eq!(sum(a, b), 3);
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    assert_eq!(vec, [5]);


    let x = "1";
    let int_x: i32 = x.parse().unwrap();
    assert_eq!(int_x, 1);
    let int_x = x.parse::<i32>().unwrap();
    assert_eq!(int_x, 1);


    let a: i32 = 0;
    let a_pos = a.is_positive();


    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");


    let point1 = Point::new(1, 2);
    let point2 = Point::new("1", "2");
    assert_eq!(point1, Point { x: 1, y: 2 });
    assert_eq!(point2, Point { x: "1", y: "2" });
}

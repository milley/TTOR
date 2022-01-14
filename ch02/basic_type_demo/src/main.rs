#![allow(dead_code, unused_variables)]
fn main() {
    let y: bool = false;
    let x = 5;
    if x > 1 { println!("x is bigger than 1")};
    let x = true;
    assert_eq!(x as i32, 1);
    assert_eq!(y as i32, 0);


    let num = 42u32;
    let num: u32 = 42;
    let num = 0x2A;
    let num = 0o106;
    let num = 0b1101_1011;
    assert_eq!(b'*', 42u8);
    assert_eq!(b'\'', 39u8);
    let num = 3.1415926f64;
    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);
    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);


    let x = 'r';
    let x = 'Ú';
    println!("{}", '\'');
    println!("{}", '\\');
    println!("{}", '\n');
    println!("{}", '\r');
    println!("{}", '\t');
    assert_eq!('\x2A', '*');
    assert_eq!('\x25', '%');
    assert_eq!('\u{CA0}', 'ಠ');
    assert_eq!('\u{151}', 'ő');
    assert_eq!('%' as i8, 37);
    assert_eq!('ಠ' as i8, -96);


    let arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [1, 2, 3];
    assert_eq!(1, mut_arr[0]);
    mut_arr[0] = 3;
    assert_eq!(3, mut_arr[0]);
    let init_err = [0; 10];
    assert_eq!(0, init_err[5]);
    assert_eq!(10, init_err.len());
    // println!("{:?}", arr[5]);



    assert_eq!((1..5), std::ops::Range{ start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3 + 4 + 5, (3..6).sum());
    assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
    for i in 1..5 {
        println!("{}", i);
    }
    for i in 1..=5 {
        println!("{}", i);
    }


    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], [2, 3, 4, 5]);
    assert_eq!(arr.len(), 5);
    assert_eq!(&arr.len(), &5);
    assert_eq!(&arr.is_empty(), &false);
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);
    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1, 2, 3]);



    let truth: &'static str = "Rust是一门优雅的语言";
    let ptr = truth.as_ptr();
    let len = truth.len();
    assert_eq!(28, len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));


    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);



    let num: Option<u32> = Some(42);
    match num {
        Some(num) => num,
        None => panic!("Nothing!"),
    };
}


// #![feature(never_type)]
fn foo() -> u32 {
    let x = {
        return 123;
    };
}

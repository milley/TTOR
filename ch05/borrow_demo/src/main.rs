// fn foo(mut v: [i32; 3]) -> [i32; 3] {
//     v[0] = 3;
//     assert_eq!([3, 2, 3], v);
//     v
// }
fn foo(v: &mut [i32; 3]) {
    v[0] = 3;
}

fn bubble_sort(a: &mut Vec<i32>) {
    let mut n = a.len();
    while n > 0 {
        let (mut i, mut max_ptr) = (1, 0);
        while i < n {
            if a[i - 1] > a[i] {
                a.swap(i - 1, i);
                max_ptr = i;
            }
            i += 1;
        }
        n = max_ptr;
    }
}


fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;
    }
}
// fn compute(input: &u32, output: &mut u32) {
//     let cached_input = *input;
//     if cached_input > 10 {
//         *output = 1;
//     }
//     if cached_input > 5 {
//         *output *= 2;
//     }
// }


// fn return_str<'a>() -> &'a str {
//     let mut s = "Rust".to_string();
//     for i in 0..3 {
//         s.push_str("Good");
//     }

//     &s[..]
// }
fn return_str() -> String {
    let mut s = "Rust".to_string();
    for i in 0..3 {
        s.push_str("Good");
    }

    s.to_string()
}


// fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}


#[derive(Debug)]
struct Foo<'a> {
    part: &'a str,
}

impl<'a> Foo<'a> {
    fn split_first(s: &'a str) -> &'a str {
        s.split(',').next().expect("Could not find a ','")
    }
    fn new(s: &'a str) -> Self {
        Foo { part: Foo::split_first(s) }
    }
    fn get_part(&self) -> &str {
        self.part
    }
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


use std::fmt::Debug;
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    // let v = [1, 2, 3];
    // foo(v);
    // assert_eq!([1, 2, 3], v);

    let mut v = [1, 2, 3];
    foo(&mut v);
    assert_eq!([3, 2, 3], v);

    let mut a = vec![1, 4, 5, 3, 2];
    bubble_sort(&mut a);
    println!("{:?}", a);


    let i = 20;
    let mut o = 5;
    compute(&i, &mut o);



    let x = return_str();


    // let x = "hello";
    // let y = "rust";
    // foo(&x, &y);


    let s1 = String::from("hello");
    let s1_r = &s1;
    {
        let s2 = String::from("C");
        let res = the_longest(s1_r, &s2);
        println!("{:?} is the longest", res);
    }


    let words = String::from("Sometimes think, the greatest sorrow than older");
    // let first = words.split(',').next().expect("Could not find a ','");
    // let f = Foo { part: first };
    // assert_eq!(f.part, "Sometimes think");
    // println!("{:?}", Foo::new(words.as_str()));
    let foo = Foo::new(words.as_str());
    println!("{:?}", foo.get_part());


    println!("{:?}", first_word("hello rust"));


    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
}

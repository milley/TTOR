fn call<F>(closure: F) -> i32
where
    F: Fn(i32) -> i32
{
    closure(1)
}
fn counter(i: i32) -> i32 { i + 1 }


trait Any {
    fn any(&self, f: &(dyn Fn(u32) -> bool)) -> bool;
}
impl Any for Vec<u32> {
    fn any(&self, f: &(dyn Fn(u32) -> bool)) -> bool {
        for &x in self.iter() {
            if f(x) {
                return true;
            }
        }
        false
    }
}


// fn square() -> Box<dyn Fn(i32) -> i32> {
//     Box::new(|i| i * i)
// }
// fn square() -> Box<dyn FnOnce(i32) -> i32> {
//     Box::new(|i| i * i)
// }

// #![feature(fnbox)]
// use std::boxed::FnBox;
// fn square() -> Box<dyn FnBox(i32) -> i32> {
//     Box::new(|i| i * i)
// }

fn square() -> impl FnOnce(i32) -> i32 {
    |i| { i * i }
}


use std::fmt::Debug;
trait DoSomething<T> {
    fn do_sth(&self, value: T);
}
impl<'a, T: Debug> DoSomething<T> for &'a usize {
    fn do_sth(&self, value: T) {
        println!("{:?}", value);
    }
}
// fn foo<'a>(b: Box<dyn DoSomething<&'a usize>>) {
//     let s: usize = 10;
//     b.do_sth(&s);
// }

fn bar(b: Box<dyn for<'f> DoSomething<&'f usize>>) {
    let s: usize = 10;
    b.do_sth(&s);
}


struct Pick<F> {
    data: (u32, u32),
    func: F,
}
// impl<F> Pick<F>
// where
//     F: Fn(&(u32, u32)) -> &u32,
// {
//     fn call(&self) -> &u32 {
//         (self.func)(&self.data)
//     }
// }
impl<F> Pick<F>
where
    F: for<'f> Fn(&'f (u32, u32)) -> &'f u32,
{
    fn call(&self) -> &u32 {
        (self.func)(&self.data)
    }
}
fn max(data: &(u32, u32)) -> &u32 {
    if data.0 > data.1 {
        &data.0
    } else {
        &data.1
    }
}


fn main() {
    let result = call(counter);
    assert_eq!(result, 2);


    let v = vec![1, 2, 3];
    let b = v.any(&|x| x == 3);
    println!("{:?}", b);


    let s = "hello";
    let c: Box<dyn Fn() + 'static> = Box::new(move || { s; });


    // let square = square();
    // assert_eq!(square(2), 4);
    // assert_eq!(square(3), 9);

    let square = square();
    assert_eq!(square(2), 4);


    let x = Box::new(&2usize);
    // foo(x);
    bar(x);


    let elm = Pick { data: (3, 1), func: max };
    println!("{}", elm.call());
}
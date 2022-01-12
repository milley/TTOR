use std::vec;

trait InIterator<T: Copy> {
    fn each<F: Fn(T) -> T>(&mut self, f: F);
}
impl<T: Copy> InIterator<T> for Vec<T> {
    fn each<F: Fn(T) -> T>(&mut self, f: F) {
        let mut i = 0;
        while i < self.len() {
            self[i] = f(self[i]);
            i += 1;
        }
    }
}


struct Counter { count: usize, }
impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}



use std::iter::FromIterator;
#[derive(Debug)]
struct MyVec(Vec<i32>);
impl MyVec {
    fn new() -> MyVec {
        MyVec(Vec::new())
    }
    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}
impl FromIterator<i32> for MyVec {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut c = MyVec::new();
        for i in iter {
            c.add(i);
        }
        c
    }
}


#[derive(Debug, Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Step<I> {
    iter: I,
    skip: usize,
}
impl<I> Iterator for Step<I>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let elt = self.iter.next();
        if self.skip > 0 {
            self.iter.nth(self.skip - 1);
        }
        elt
    }
}
pub fn step<I>(iter: I, step: usize) -> Step<I>
where
    I: Iterator,
{
    assert!(step != 0);
    Step { iter: iter, skip: step - 1, }
}
pub trait IterExt: Iterator {
    fn step(self, n: usize) -> Step<Self>
    where
        Self: Sized,
    {
        step(self, n)
    }
}
impl<T: ?Sized> IterExt for T where T: Iterator {}



fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("{}", i);
    }



    let mut v = vec![1, 2, 3, 4, 5];
    v.each(|i| i * 3);
    assert_eq!(&v[..5], [3, 6, 9, 12, 15]);


    let v = vec![1, 2, 3, 4, 5];
    {
        let mut _iterator = v.into_iter();
        loop {
            match _iterator.next() {
                Some(i) => {
                    println!("{}", i);
                }
                None => break,
            }
        }
    }


    let mut counter = Counter { count: 0 };
    assert_eq!(Some(1), counter.next());
    assert_eq!(Some(2), counter.next());
    assert_eq!(Some(3), counter.next());
    assert_eq!(Some(4), counter.next());
    assert_eq!(Some(5), counter.next());
    assert_eq!(None, counter.next());



    let a: [i32; 3] = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());
    iter.next();
    assert_eq!((2, Some(2)), iter.size_hint());


    let mut message = "Hello".to_string();
    message.extend(&[' ', 'R', 'u', 's', 't']);
    assert_eq!("Hello Rust", &message);



    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("{:?}", i);
    }
    println!("{:?}", arr);


    let mut arr = [1, 2, 3, 4, 5];
    for i in arr.iter_mut() {
        *i += 1;
    }
    println!("{:?}", arr);


    let a = [1, 2, 3];
    let mut iter = a.into_iter().map(|x| x * 2);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);


    let arr1 = [1, 2, 3, 4, 5];
    let c1 = arr1.iter().map(|x| 2 * x).collect::<Vec<i32>>();
    assert_eq!(&c1[..], [2, 4, 6, 8, 10]);
    let arr2 = ["1", "2", "3", "h"];
    let c2 = arr2.iter().filter_map(|x| x.parse().ok()).collect::<Vec<i32>>();
    assert_eq!(&c2[..], [1, 2, 3]);
    let arr3 = ['a', 'b', 'c'];
    for (idx, val) in arr3.iter().enumerate() {
        println!("idx: {:?}, val: {}", idx, val.to_uppercase());
    }


    let a = [1, 2, 3];
    let mut iter = a.iter().rev();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);



    let numbers = vec![1, 2, 3, 4, 5, 6];
    let mut iter = numbers.into_iter();
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(6), iter.next_back());
    assert_eq!(Some(5), iter.next_back());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(4), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());


    let a = [1, 2, 3];
    assert_eq!(a.iter().any(|&x| x != 2), true);
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);


    let arr = [1, 2, 3];
    let result1 = arr.iter().any(|&x| x != 2);
    let result2 = arr.iter().any(|x| *x != 2);
    // let result2 = arr.iter().any(|x| x != 2);
    assert_eq!(result1, true);
    assert_eq!(result2, true);


    let arr = vec![1, 2, 3];
    let sum1 = arr.iter().fold(0, |acc, x| acc + x);
    let sum2 = arr.iter().fold(0, |acc, x| acc + *x);
    let sum3 = arr.iter().fold(0, |acc, &x| acc + x);
    let sum4 = arr.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum1, 6);
    assert_eq!(sum2, 6);
    assert_eq!(sum3, 6);
    assert_eq!(sum4, 6);


    let iter = (0..5).into_iter();
    let c = MyVec::from_iter(iter);
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
    let iter = (0..5).into_iter();
    let c: MyVec = iter.collect();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
    let iter = (0..5).into_iter();
    let c = iter.collect::<MyVec>();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);



    let arr = [1, 2, 3, 4, 5, 6];
    let sum = arr.iter().step(2).fold(0, |acc, x| acc + x);
    assert_eq!(sum, 9);
}

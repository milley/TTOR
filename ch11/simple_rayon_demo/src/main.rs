use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}
fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|p| *p += 1);
}
fn fib(n: u32) -> u32 {
    if n < 2 { return n; }
    let (a, b) = rayon::join(
        || fib(n - 1), || fib(n - 2)
    );
    a + b
}
fn main() {
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r = sum_of_squares(&v);
    println!("{}", r);
    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    increment_all(&mut v);
    println!("{:?}", v);


    let r = fib(32);
    assert_eq!(r, 2178309);
}

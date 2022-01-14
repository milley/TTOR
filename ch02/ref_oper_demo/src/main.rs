fn main() {
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p}", b);

    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:p}", d);
    let e = &42;
    assert_eq!(42, *e);
}

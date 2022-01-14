pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn is_true() -> bool { true }
fn true_maker() -> fn() -> bool { is_true }


const fn init_len() -> usize {
    return 5;
}

fn main() {
    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);


    assert_eq!(true_maker()(), true);


    let _arr = [0; init_len()];
}

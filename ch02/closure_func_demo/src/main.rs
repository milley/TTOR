fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}


fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}

fn main() {
    let out = 42;
    fn add(i: i32, j: i32) -> i32 { i + j }
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i, j| i + j + out;
    let i = 1;
    let j = 2;
    assert_eq!(3, add(i, j));
    assert_eq!(45, closure_annotated(i, j));
    assert_eq!(45, closure_inferred(i, j));


    let a = 2;
    let b = 3;
    assert_eq!(closure_math(||a + b), 5);
    assert_eq!(closure_math(||a * b), 6);


    let result = two_times_impl();
    assert_eq!(result(2), 4);
}

fn call<F: FnOnce()>(f: F) { f() }


fn boxed_closure(c: &mut Vec<Box<dyn Fn()>>) {
    let s = "second";
    c.push(Box::new(|| println!("first")));
    c.push(Box::new(move || println!("{}", s)));
    c.push(Box::new(|| println!("third")));
}


use std::ops::Fn;
trait Any {
    fn any<F>(&self, f: F) -> bool
    where
        Self: Sized,
        F: Fn(u32) -> bool;
}
impl Any for Vec<u32> {
    fn any<F>(&self, f: F) -> bool
    where
        Self: Sized,
        F: Fn(u32) -> bool
    {
        for &x in self {
            if f(x) {
                return true;
            }
        }
        false
    }
}




fn main() {
    let mut x = 0;
    let incr_x = || x += 1;
    call(incr_x);
    // call(incr_x);
    let mut x = 0;
    let incr_x = move || x += 1;
    call(incr_x);
    call(incr_x);
    let mut x = vec![];
    let expend_x = move || x.push(42);
    call(expend_x);
    // call(expend_x);


    let mut s = "rush".to_string();
    {
        let mut c = || { s += " rust"; };
        c();
        c();
        println!("{}", s);
    }
    println!("{}", s);


    let c = || { println!("hhh"); };
    c();
    c();


    let mut c: Vec<Box<dyn Fn()>> = vec![];
    boxed_closure(&mut c);
    for f in c { f(); }


    let v = vec![1, 2, 3];
    let b = v.any(|x| x == 3);
    println!("{:?}", b);
    
}

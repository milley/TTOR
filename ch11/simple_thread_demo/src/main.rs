use std::thread;
// use std::rc::Rc;
use std::sync::{Arc, Mutex};
// use std::cell::RefCell;

fn main() {
    // let mut s = "Hello".to_string();
    // for _ in 0..3 {
    //     thread::spawn(move || {
    //         s.push_str(" Rust!");
    //     });
    // }

    // let mut s = Rc::new("Hello".to_string());
    // for _ in 0..3 {
    //     let mut s_clone = s.clone();
    //     thread::spawn(move || {
    //         s_clone.push_str(" hello");
    //     });
    // }

    // let s = Arc::new("Hello".to_string());
    // for _ in 0..3 {
    //     let s_clone = s.clone();
    //     thread::spawn(move || {
    //         s_clone.push_str(" world!");
    //     });
    // }

    let s = Arc::new(Mutex::new("Hello".to_string()));
    let mut v = vec![];
    for _ in 0..3 {
        let s_clone = s.clone();
        let child = thread::spawn(move || {
            let mut s_clone = s_clone.lock().unwrap();
            s_clone.push_str(" world!");
        });
        v.push(child);
    }

    for child in v {
        child.join().unwrap();
    }

}

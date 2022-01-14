use std::sync::{Arc, Barrier};
use std::thread;


fn main() {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    for _ in 0..5 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before await");
            c.wait();
            println!("after await");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


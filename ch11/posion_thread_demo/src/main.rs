use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(1));
    let c_mutex = mutex.clone();
    let _ = thread::spawn(move || {
        let mut data = c_mutex.lock().unwrap();
        *data = 2;
        panic!("oh no");
    }).join();
    assert_eq!(mutex.is_poisoned(), true);
    match mutex.lock() {
        Ok(_) => unreachable!(),
        Err(p_err) => {
            let data = p_err.get_ref();
            println!("recovered: {}", data);
        }
    };
}

use crossbeam::thread::scope;
use crossbeam::channel as channel;


fn main() {
    let array = [1, 2, 3];
    let mut guards = vec![];
    for &i in &array {
        let guard = std::thread::spawn(move || {
            println!("element: {}", i);
        });
        guards.push(guard);
    }
    for guard in guards {
        guard.join().unwrap();
    }


    let array = [1, 2, 3];
    scope(|scope| {
        for i in &array {
            scope.spawn(move |_| { println!("element: {}", i); });
        }
    }).unwrap();


    let (s, r) = channel::unbounded();
    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            s.send(1).unwrap();
            r.recv().unwrap();
        });
        scope.spawn(|_| {
            s.send(2).unwrap();
            r.recv().unwrap();
        });
    }).unwrap();
}

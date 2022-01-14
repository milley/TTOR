// use crossbeam_channel as channel;
// use crossbeam_channel::{select, unbounded};
// use std::thread;

// fn fibonacci(fib: channel::Sender<u64>, quit: channel::Receiver<()>) {
//     let (mut x, mut y) = (0, 1);
//     loop {
//         select! {
//             send(fib, x) => {
//                 let tmp = x;
//                 x = y;
//                 y = tmp + x;
//             }
//             recv(quit) => {
//                 println!("quit");
//                 return;
//             }
//         }
//     }
// }

// fn main() {
//     let (fib_s, fib_r) = channel::bounded(0);
//     let (quit_s, quit_r) = channel::bounded(0);
//     thread::spawn(move || {
//         for _ in 0..10 {
//             println!("{}", fib_r.recv().unwrap());
//         }
//         quit_s.send(());
//     });
//     fibonacci(fib_s, quit_r);
// }


// use crossbeam_channel::{select, unbounded};
// fn main() {
//     let (s1, r1) = unbounded();
//     let (s2, r2) = unbounded();
//     s1.send(10).unwrap();
    
//     // Since both operations are initially ready, a random one will be executed.
//     select! {
//         recv(r1) -> msg => assert_eq!(msg, Ok(10)),
//         send(s2, 20) -> res => {
//             assert_eq!(res, Ok(()));
//             assert_eq!(r2.recv(), Ok(20));
//         }
//     }
// }

fn main() {
    
}
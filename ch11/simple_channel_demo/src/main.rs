use std::thread;
use std::sync::mpsc::{channel, sync_channel};


fn main() {
    // let (tx, rx) = channel();
    // thread::spawn(move || {
    //     tx.send(10).unwrap();
    // });
    // assert_eq!(rx.recv().unwrap(), 10);


    // let (tx, rx) = channel();
    // for i in 0..10 {
    //     let tx = tx.clone();
    //     thread::spawn(move || {
    //         tx.send(i).unwrap();
    //     });
    // }
    // for _ in 0..10 {
    //     let j = rx.recv().unwrap();
    //     assert!(0 <= j && j < 10);
    // }

    // let (tx, rx) = sync_channel(1);
    // tx.send(1).unwrap();
    // thread::spawn(move || {
    //     tx.send(2).unwrap();
    // });
    // assert_eq!(rx.recv().unwrap(), 1);
    // assert_eq!(rx.recv().unwrap(), 2);


    // let (tx, rx) = channel();
    // for i in 0..5 {
    //     let tx = tx.clone();
    //     thread::spawn(move || {
    //         tx.send(i).unwrap();
    //     });
    // }

    // // drop(tx);
    // for j in rx.iter() {
    //     println!("{:?}", j);
    // }


    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(1u8).unwrap();
        tx.send(2u8).unwrap();
        tx.send(3u8).unwrap();
    });
    for x in rx.iter() {
        println!("receive: {}", x);
    }
}

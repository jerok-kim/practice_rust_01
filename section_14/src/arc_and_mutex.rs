use std::rc::Rc;
use std::thread;
// use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};

fn main() {
    let handle = thread::spawn(move || {
        println!("Hello from a thread!");
    });
    // thread::sleep(Duration::from_secs(1));

    handle.join().unwrap();
    println!("Hello from main!");

    let v = vec![1, 2, 3];
    // let join_handle = thread::spawn(move || {
    //     println!("{:?}", v);
    // });

    let mut thread_handles = Vec::new();
    for e in v {
        thread_handles.push(thread::spawn(move || println!("Thread {}", e)));
    }

    println!("Main thread!");

    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("=============================");

    let (transmitter, receiver) = mpsc::sync_channel(1000);
    let tx = transmitter.clone();

    // let val = String::from("Transmitting!");
    // thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // let msg = receiver.recv().unwrap();
    // println!("{}", msg);

    thread::spawn(move || {
        let vec = vec![
            String::from("Transmitting"),
            String::from("from"),
            String::from("Original"),
        ];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vec = vec![
            String::from("Clone"),
            String::from("is"),
            String::from("Transmitting"),
        ];
        for val in vec {
            tx.send(val).unwrap();
        }
    });

    for rec in receiver {
        println!("{}", rec);
    }

    let rc1 = Arc::new(String::from("test"));
    let rc2 = rc1.clone();

    thread::spawn(move || {
        rc2;
    });

    println!("=============================");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}

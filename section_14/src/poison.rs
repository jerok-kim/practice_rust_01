use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock1);

    let _ = thread::spawn(move || -> () {
        let _guard = lock2.lock().unwrap();  // we acquire the lock here
        panic!();  // mutex is now poisoned
    }).join();

    let mut guard = match lock1.lock() {
        Ok(guard)=>guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *guard += 1;
    println!("{:?}", guard);
}

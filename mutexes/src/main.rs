use std::sync::{Mutex, Arc};
use std::thread::{self, JoinHandle};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..10 {
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

    println!("Result: {}", *counter.lock().unwrap());
}

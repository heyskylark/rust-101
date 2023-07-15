use std::sync::{Mutex, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/**
 * This is an example of a deadlock, running this will cause the program to freeze and will need to be force quit.
 */
fn main() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for index in 0..10 {
        let counter1 = Arc::clone(&counter1);
        let counter2 = Arc::clone(&counter2);

        let handle = thread::spawn(move || {
            if index % 2 == 0 {
                let mut num1 = counter1.lock().unwrap();
                thread::sleep(Duration::from_millis(1));
                let mut num2 = counter2.lock().unwrap();

                *num1 += 1;
                *num2 += 1;
            } else {
                let mut num2 = counter2.lock().unwrap();
                thread::sleep(Duration::from_millis(1));
                let mut num1 = counter1.lock().unwrap();

                *num1 += 1;
                *num2 += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Done: {}, {}", *counter1.lock().unwrap(), *counter2.lock().unwrap());
}

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            String::from("Sender 1: Hello, this is msg 1"),
            String::from("Sender 1: This is msg 2, did you get the last?"),
            String::from("Sender 1: This is my final message, so long.")
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("Sender 2: Hello, this is sender 2 speaking"),
            String::from("Sender 2: Heyo, did you get the last msg?"),
            String::from("Sender 2: Okay, this is my final msg, so long.")
        ];

        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(1200));
        }
    });

    for recieved in rx {
        println!("Msg: {recieved}");
    }
}

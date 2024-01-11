use std::sync::mpsc;
use std::thread;

fn main() {
    // tx=transmitter, rx=receiver
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel(); 

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // thread sends ownership of val to main
    });

    let received = rx.recv().unwrap(); //recv waits until a value is received or when transmitter closes
    println!("Got: {}", received);
}
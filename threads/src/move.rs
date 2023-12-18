use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // use move to take ownership of v variable
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
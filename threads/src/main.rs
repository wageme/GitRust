use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread says {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // Wait for handle to finish before continuing

    for i in 1..5 {
        println!("main thread says {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Wait for handle to finish before finishing

}

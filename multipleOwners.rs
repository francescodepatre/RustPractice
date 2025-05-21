use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let five = Arc::new(String::from("Number five"));
    for _ in 0..10 {
        let five_ref = Arc::clone(&five);
        thread::spawn(move || {
            println!("Hello from thread! {}", five_ref);
        });
    }
}

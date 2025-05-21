use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let m_ref = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut counter = m_ref.lock().unwrap();
            *counter += 1;
            println!("Counter: {}", *counter);
        });
        handles.push(handle);
    }
}

use std::thread;

fn main() {
    let m = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut counter = m.lock().unwrap();
            *counter += 1;
        });
        handles.push(handle);
    }
}

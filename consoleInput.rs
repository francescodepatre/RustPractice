fn main() {
    use std::io::*;
    println!("What is your name?");
    let name = stdin().lock().lines().next().unwrap().unwrap();
    println!("Hello, {}!", name);

    for l in stdin().lock().lines() {
        println!("{}", l.unwrap().to_uppercase());
    }
}

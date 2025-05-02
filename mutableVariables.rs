fn main() {
    let mut x = 1;
    println!("x = {}", x);
    x = x + 1; // This will not cause a compile-time error
    println!("x = {}", x);
    let y = 6;
    let _y2 = y.to_string();
    println!("y = {}", _y2);
}

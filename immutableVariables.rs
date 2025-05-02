fn main() {
    let x = 1;
    println!("x = {}", x);
    x = x + 1; // This will cause a compile-time error
    println!("x = {}", x);
}

const MONTH_NUMBER: u8 = 12; // This is a constant

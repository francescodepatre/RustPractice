fn main() {
    let x = 5;
    let alias = &x;

    x = 6;
    println!("x: {}, alias: {}", x, alias); // This will cause a compile-time error because `x` is immutable
}

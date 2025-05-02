fn main() {
    let s = String::from("hello World!");
    let hello: &str = &s[0..5]; // hello
    let world: &str = &s[6..11]; // World!
    println!("{}\n{}", hello, world);
}

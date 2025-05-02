fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let val10: Option<&i32> = v.get(10);
    println!("none? {}", val10.is_none());
    let val10: i32 = v[10]; // This will panic
    println!("val10: {}", val10);
}

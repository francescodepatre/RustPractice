fn main() {
    let a = [1, 2, 3, 4, 5]; // This is an array
    println!("First element of the array: {}", a[0]); // This will print 1

    let mut b = [[0u8; 4]; 6];
    b[0][1] = 42;
}

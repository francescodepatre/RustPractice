fn abs(x: i32) -> i32 {
    if x >= 0 {
        return x;
    } else {
        return -x;
    }
}

fn add_three(x: i32) -> i32 {
    x + 3
}

fn main() {
    let x = -5;
    let y = 10;
    let z = add_three(x);
    println!("The absolute value of {} is {}", x, abs(x));
    println!("The absolute value of {} is {}", y, abs(y));
    println!("Add three function for {} is {}", x, z);
}

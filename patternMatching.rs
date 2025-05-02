fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let x = Some(5);
    let y = plus_one(x);
    println!("y = {:?}", y);
    let z = plus_one(None);
    println!("z = {:?}", z);
}

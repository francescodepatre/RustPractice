fn main() {
    let val = 1;
    let closure_annotated = |i: i32| -> i32 { i + val };
    let closure_inferred = |i| i + val;

    let j = 2;
    println!("closure_annotated: {}", closure_annotated(j));
    println!("closure_inferred: {}", closure_inferred(j));

    let one = || 1;
    println!("closure returning one: {}", one());
}

fn main() {
    let tupl_expl: (i32, f64, u8, char) = (500, 6.4, 1, 'a');
    let tupl_impl = (500, true);
    println!(
        "Tuple: {},{},{},{}",
        tupl_expl.0, tupl_expl.1, tupl_expl.2, tupl_expl.3
    );
    let (a, b) = tupl_impl; // destructuring
    println!("Tuple destructuring: {},{}", a, b);
}

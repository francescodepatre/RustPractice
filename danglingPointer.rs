/*
fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
*/
fn noDangle() -> String {
    let s = String::from("Hello");
    s
}

fn main() {
    let reference_to_nothing = noDangle();
}

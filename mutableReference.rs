fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut str = String::from("Hello");
    change(&mut str);
    println!("{}", str);
}

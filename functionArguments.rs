fn addFive(someInteger: i32) -> i32 {
    someInteger + 5
}

fn takeOwnership(someString: String) {
    println!("String: {}", someString);
}

fn takeAndReturnOwnership(someString: String) -> String {
    println!("String: {}", someString);
    someString
}

fn concat2(s1: String, s2: String) -> (String, String, String) {
    let mut resultStr = String::new();
    resultStr.push_str(&s1);
    resultStr.push_str(&s2);
    (resultStr, s1, s2)
}

fn main() {
    let s = String::from("Hello");
    takeOwnership(s);
    //println!("String: {}", s); // This will cause a compile-time error because `s` has been moved

    let x = 5;
    let y = addFive(x);
    println!("x: {}, y: {}", x, y); // This will work because `x` is a copy type
}

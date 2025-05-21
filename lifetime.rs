struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("long string is long");
    let result;

    let str2 = String::from("xyz");
    result = longest(str1.as_str(), str2.as_str());

    println!("The longest string is: {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let imp = ImportantExcerpt {
        part: first_sentence,
    };
}

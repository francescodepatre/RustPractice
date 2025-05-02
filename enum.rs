enum IPAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IPAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    Some(T),
    None,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a String");
    let absent_number: Option<i32> = None;
    let x = some_number.unwrap_or(0) + 3;
}

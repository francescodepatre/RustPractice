struct User {
    username: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("francesco"),
        sign_in_count: 1,
        active: true,
    };

    println!(
        "User1: {}, {}, {}",
        user1.username, user1.active, user1.sign_in_count
    );
}

fn first_structure() {
    let mut x = 5;
    while x > 0 {
        println!("x is {}", x);
        x -= 1;
    }
}

fn second_structure() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("The value is: {}", number);
    }
}

fn main() {
    first_structure();
    second_structure();
}

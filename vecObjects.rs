pub enum Animal {
    Dog(String),
    Cat(String),
    Pig(String),
}
impl Animal {
    pub fn speak(&self) {
        match self {
            Animal::Dog(name) => println!("{} says Woof!", name),
            Animal::Cat(name) => println!("{} says Meow!", name),
            Animal::Pig(name) => println!("{} says Oink!", name),
        }
    }
}

fn main() {
    let v = vec![
        Animal::Dog(String::from("Danny")),
        Animal::Cat(String::from("Candy")),
        Animal::Pig(String::from("Peppa")),
    ];

    for a in v {
        a.speak();
    }
}

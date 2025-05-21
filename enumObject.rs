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

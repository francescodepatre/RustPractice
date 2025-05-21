pub struct Dog {
    name: String,
}
impl Dog {
    pub fn new(name: String) -> Dog {
        Dog {
            name: name.to_string(),
        }
    }
}
impl Animal for Dog {
    fn speak(&self) {
        println!("{} says Woof!", self.name);
    }
}

pub trait Animal {
    fn speak(&self);
}

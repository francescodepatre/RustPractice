pub struct Farm<T: Animal> {
    pub animals: Vec<T>,
}
impl<T: Animal> Farm<T> {
    pub fn run(&self) {
        for animal in &self.animals {
            animal.speak();
        }
    }
}

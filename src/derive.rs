use std::fmt::{Debug, Formatter};

struct Person {
    name: String,
    age: u8,
}

impl Debug for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{ name: {:?}, age: {}}}", self.name, self.age)
    }
}

pub fn run() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{:?}", person)
}

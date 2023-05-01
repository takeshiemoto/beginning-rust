struct Person {
    name: String,
    age: u32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

pub fn run() {
    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let person2 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let person3 = Person {
        name: "Bob".to_string(),
        age: 25,
    };

    println!("person1 == person2: {}", person1 == person2);
    println!("person1 == person3: {}", person1 == person3);
}

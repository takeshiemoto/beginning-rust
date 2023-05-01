#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Yamada Taro".to_string(),
            age: 20,
        }
    }
}

pub fn run() {
    let person = Person::default();
    println!("{:?}", person);
}

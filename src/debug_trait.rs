use std::fmt::{Debug, Formatter};

struct Person {
    name: String,
    age: u32,
}

// 特別な出力が必要なケース以外はアトリビュートを使うと良い
// #[derive(Debug)]
impl Debug for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}

pub fn run() {
    let person = Person {
        name: "Ichiro".to_string(),
        age: 45,
    };

    println!("{:?}", person);
}

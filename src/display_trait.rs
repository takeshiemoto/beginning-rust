use std::fmt::{Display, Formatter};

pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name:{}, Age: {}", self.name, self.age)
    }
}

pub struct BaseballPlayer {
    name: String,
    position: String,
    batting_average: f32,
}

pub fn run() {
    let person = Person {
        name: "Ichiro".to_string(),
        age: 44,
    };

    println!("{}", person); // Name:Ichiro, Age: 44
    println!("{}", person.to_string()); // Name:Ichiro, Age: 44

    let baseball_player = BaseballPlayer {
        name: "Ichiro".to_string(),
        position: "Right".to_string(),
        batting_average: 0.366,
    };

    // Displayトレイトが実装されていないので毎度書かないといけない
    println!(
        "Name {}, Position: {}, Average {}",
        baseball_player.name, baseball_player.position, baseball_player.batting_average
    );
}

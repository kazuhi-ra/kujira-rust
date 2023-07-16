#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn introduce_myself(&self) {
        println!("I am {}, {} years old.", self.name, self.age)
    }

    fn turn_one_year_old(self) -> Self {
        Self {
            age: self.age + 1,
            ..self
        }
    }
}

fn main() {
    let kirito = User::new("kirito", 16);
    kirito.introduce_myself();
    let kirito = kirito.turn_one_year_old();
    User::introduce_myself(&kirito);
}

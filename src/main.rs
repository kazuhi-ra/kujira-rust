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

    fn give_ones_name(&self) {
        println!("I am {}.", self.name)
    }
}

fn main() {
    let kirito = User::new("kirito", 16);
    kirito.give_ones_name()
}

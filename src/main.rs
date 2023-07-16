#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        User { name, age }
    }

    fn give_ones_name(&self) {
        println!("I am {}.", self.name)
    }
}

fn main() {
    let kirito = User::new(String::from("kirito"), 16);
    kirito.give_ones_name()
}

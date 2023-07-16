#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn give_ones_name(&self) {
        println!("I am {}.", self.name)
    }
}

fn main() {
    let kirito = User {
        name: String::from("kirito"),
        age: 16,
    };

    kirito.give_ones_name()
}

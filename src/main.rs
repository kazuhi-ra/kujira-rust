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

    fn turn_one_year_old(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut kirito = User::new("kirito", 16);
    kirito.introduce_myself();
    kirito.turn_one_year_old();
    kirito.introduce_myself()
}

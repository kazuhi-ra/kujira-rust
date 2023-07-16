mod util {
    pub fn fizz_buzz() {
        for i in 1..=50 {
            let result = match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => format!("{}", i),
            };

            println!("{}", result)
        }
    }
}

use util::fizz_buzz;
fn main() {
    fizz_buzz()
}

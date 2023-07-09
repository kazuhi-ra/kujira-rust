fn main() {
    for a in 1..10 {
        for b in 1..10 {
            print!("{:3}", a * b);
            if b != 9 {
                print!(",");
            }
        }
        println!("");
    }
}

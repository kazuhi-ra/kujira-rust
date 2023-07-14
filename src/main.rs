fn main() {
    let a = "kazuhi-ra".to_string();

    {
        let b = a;
    }

    println!("{}", a);
}

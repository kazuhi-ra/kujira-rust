fn main() {
    let a = "kazuhi-ra".to_string();

    print_string(&a);
    println!("{}", a);
}

fn print_string(s: &String) -> () {
    println!("{}", s);
}

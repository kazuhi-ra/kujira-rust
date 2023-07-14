fn main() {
    let a = "kazuhi-ra".to_string();

    let a = print_string(a);
    println!("{}", a);
}

fn print_string(s: String) -> String {
    println!("{}", s);
    s
}

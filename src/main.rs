fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("plz file name");
        return;
    }

    let file_name = &args[1];

    let txt = match std::fs::read_to_string(file_name) {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    println!("{txt}")
}

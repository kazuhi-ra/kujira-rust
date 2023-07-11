fn main() {
    let args = std::env::args();
    let mut result = 0.0;

    for (i, s) in args.enumerate() {
        if i == 0 {
            continue;
        }

        let num = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };

        result += num;
    }

    println!("total: {}", result)
}

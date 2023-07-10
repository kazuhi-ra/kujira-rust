use std::io::stdin;

fn main() {
    let height_cm = input_f(0.0);
    let weight = input_f(0.0);
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("bmi: {}", bmi);
}

fn input_str() -> String {
    let mut s = String::new();

    match stdin().read_line(&mut s) {
        Ok(_) => s.trim_end().to_string(),
        Err(_) => "入力エラー".to_string(),
    }
}

fn input_f(def: f64) -> f64 {
    println!("{}", def);
    let s = input_str();

    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}

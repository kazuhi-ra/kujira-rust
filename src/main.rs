fn main() {
    let a = add_i32(-10, 42);
    println!("{}", a);

    let b = add_u64(10, 42);
    println!("{}", b);
}

fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_u64(a: u64, b: u64) -> u64 {
    a + b
}

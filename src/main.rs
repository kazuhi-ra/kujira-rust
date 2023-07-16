fn main() {
    let a = add(-10, 42_i64);
    println!("{}", a);

    let b = add(10_u8, 42);
    println!("{}", b);
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

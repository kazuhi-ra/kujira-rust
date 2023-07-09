fn fb(i: u32) -> u32 {
    if i == 1 || i == 2 {
        1
    } else {
        fb(i - 1) + fb(i - 2)
    }
}

fn main() {
    for i in 1..31 {
        println!("{:3}: {}", i, fb(i))
    }
}

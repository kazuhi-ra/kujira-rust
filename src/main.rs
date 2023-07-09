fn main() {
    for i500 in 0u16..11 {
        for i100 in 0u16..4 {
            for i50 in 0u16..11 {
                let amount = 500 * i500 + 100 * i100 + 50 * i50;
                if amount == 3950 {
                    println!("500円: {}枚, 100円: {}枚, 50円: {}枚", i500, i100, i50);
                }
            }
        }
    }
}

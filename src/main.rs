enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
    Coin50(isize),
    Coin100(isize),
    Coin500(isize),
}

impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1(v) => 1 * v,
            Coin::Coin5(v) => 5 * v,
            Coin::Coin10(v) => 10 * v,
            Coin::Coin50(v) => 50 * v,
            Coin::Coin100(v) => 100 * v,
            Coin::Coin500(v) => 500 * v,
        }
    }
}

fn main() {
    let wallet: [Coin; 6] = [
        Coin::Coin1(3),
        Coin::Coin5(10),
        Coin::Coin10(5),
        Coin::Coin50(1),
        Coin::Coin100(1),
        Coin::Coin500(5),
    ];

    let total = wallet
        .iter()
        .fold(0, |sum, current| sum + current.calc_price());

    println!("{}", total)
}

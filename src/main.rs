fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_primses(primes: &mut [usize; 100]) -> () {
    let mut i = 2;
    let mut count = 0;

    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut primes = [0; 100];
    get_primses(&mut primes);
    println!("{:?}", primes);

    let mut v: u32 = 10;
    set_value(&mut v);
    print!("{}", v);
}

fn set_value(arg: &mut u32) {
    *arg = 4649;
}

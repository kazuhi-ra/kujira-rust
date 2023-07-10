use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    let mut maze = [[false; MAP_N]; MAP_N];

    // 外壁
    for n in 0..MAP_N {
        let row = &mut maze[n];
        if n == 0 || n == MAP_N - 1 {
            row.fill(true);
        } else {
            row[0] = true;
            row[MAP_N - 1] = true;
        }
    }

    for n in 2..(MAP_N - 2) {
        if n % 2 == 0 {
            for m in 2..(MAP_N - 2) {
                if m % 2 == 0 {
                    // マス配置
                    maze[n][m] = true;
                    // 倒す
                    match randam() {
                        0 => maze[n - 1][m] = true,
                        1 => maze[n][m + 1] = true,
                        2 => maze[n + 1][m] = true,
                        3 => maze[n][m - 1] = true,
                        _ => {}
                    }
                }
            }
        }
    }

    // print
    for n in 0..MAP_N {
        let row = maze[n];
        for e in row {
            let result = if e { '❌' } else { '　' };
            print!("{}", result);
        }
        println!("");
    }
}

fn randam() -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..=3);
}

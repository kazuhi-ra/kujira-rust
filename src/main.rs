use rand::seq::SliceRandom;

fn main() {
    let mut nums: Vec<u8> = vec![];
    for i in 1..=75 {
        nums.push(i)
    }

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for i in 0..25 {
        let x = if i == 12 {
            "*".to_string()
        } else {
            nums[i].to_string()
        };
        print!("{:3}", x);
        if i % 5 == 4 {
            println!("")
        }
    }
}

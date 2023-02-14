use std::collections::HashMap;
use std::io;

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn solve(coins: Vec<u64>, price: u64) {
    let mut solutions: HashMap<u64, u64> = HashMap::new();
    solutions.insert(0, 0);
    let limit = price + coins.iter().max().unwrap() + 1;

    for coin in coins.iter() {
        for (val, count) in solutions.clone().iter() {
            let new_val = val + coin;
            if new_val > limit {
                continue;
            }

            let new_count = count + 1;
            if !solutions.contains_key(&new_val) || solutions[&new_val] > new_count {
                solutions.insert(new_val, new_count);
            }
        }
    }

    for i in price..limit {
        if solutions.contains_key(&i) {
            println!("{} {}", i, solutions[&i]);
            return;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let t = get_input(&stdin).trim().parse::<u64>().unwrap();

    for _ in 0..t {
        let price = get_input(&stdin).trim().parse::<u64>().unwrap();
        let coint_count = get_input(&stdin).trim().parse::<u64>().unwrap();
        let mut coins = Vec::new();

        for _ in 0..coint_count {
            coins.push(get_input(&stdin).trim().parse::<u64>().unwrap());
        }

        solve(coins, price);
    }
}

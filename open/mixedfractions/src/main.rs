use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    loop {
        let nums : Vec<u64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();

        if nums[0] == 0 && nums[1] == 0 {
            break;
        }

        println!("{} {} / {}", nums[0] / nums[1], nums[0] % nums[1], nums[1]);
    }
}
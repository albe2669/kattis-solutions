use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> i64 {
    return get_input(io).trim().parse::<i64>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let t = get_input_int(&stdin);
    
    //let mut nums: Vec<i64> = Vec::new();
    for _i in 0..t {
        let nums: Vec<i64> = get_input(&stdin).split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        
        let mut count : i64 = 0;
        for i in 1..nums.len() {
            let num : i64 = nums[i]-nums[i-1] * 2;

            if num > 0 {
                count += num;
            }
        }

        println!("{}", count);
    }
}
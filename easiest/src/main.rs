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
    let mut n = get_input_int(&stdin);
    
    //let mut nums: Vec<i64> = Vec::new();
    while n != 0 {
        let mut i = 10;
        let n_sum : u32 = n
            .to_string()
            .chars()
            .map(|s| s.to_digit(10).unwrap())
            .sum();

        loop {
            i += 1;

            let sum : u32 = (i * n)
                .to_string()
                .chars()
                .map(|s| s.to_digit(10).unwrap())
                .sum();

            if sum == n_sum {
                println!("{}", i);
                break;
            }
        }


        n = get_input_int(&stdin);
    }
}

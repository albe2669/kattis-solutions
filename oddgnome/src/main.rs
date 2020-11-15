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
    let n = get_input_int(&stdin);
    
    //let mut nums: Vec<i64> = Vec::new();
    for _i in 0..n {
        let t : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        for i in 2..t.len() {
            if t[i - 1] + 1 != t[i] {
                println!("{}", i);
                break;
            }
        }

    }
}

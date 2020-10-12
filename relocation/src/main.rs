use std::io::{self};
use std::collections::HashMap;

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let nq : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
    let mut companies : HashMap<u32, i64> = HashMap::new();

    let c : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();

    for i in 0..c.len() {
        companies.insert(i as u32 + 1, c[i] as i64);
    }

    for _i in 0..nq[1] {
        let query : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        
        if query[0] == 1 {
            companies.insert(query[1], query[2]  as i64);
        } else {
            println!("{}", (companies.get(&query[1]).unwrap() - companies.get(&query[2]).unwrap()).abs());
        }
    }
}
use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> u32 {
    return get_input(io).trim().parse::<u32>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let n = get_input_int(&stdin);
    let mut days : Vec<u32> = Vec::new();
    
    for _i in 0..n {
        let t : Vec<u32> = get_input(&stdin).split_whitespace().map(|n| n.parse().unwrap()).collect();

        for j in t[0]..t[1]+1 { 
            if !days.contains(&j) {
                days.push(j);
            }
        }
    }

    println!("{}", days.len());
}
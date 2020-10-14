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

    let v : u32 = get_input_int(&stdin);
    
    println!("{}", calc(v));
}

fn calc(n : u32) -> u32 {
    if 10 > n {
        return n;
    }

    let mut res : u32 = 1;
    
    for i in n.to_string().chars().collect::<Vec<char>>() {
        if i == '0' {
            continue;
        }

        res *= i.to_string().parse::<u32>().unwrap();
    }

    return calc(res);
}
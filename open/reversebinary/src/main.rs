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

    let bin : String = format!("{:b}", get_input_int(&stdin)).chars().rev().collect();
    let int : u32 = u32::from_str_radix(&bin, 2).unwrap();
    
    println!("{}", int);
}
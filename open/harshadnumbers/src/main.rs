use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> u32 {
    return get_input(io).trim().parse::<u32>().unwrap();
}

fn digit_sum(n : &u32) -> u32 {
    return n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
}

fn main() {
    let stdin = io::stdin();
    let mut n : u32 = get_input_int(&stdin);
    
    while n % digit_sum(&n) > 0 {
        n += 1;
    }

    println!("{}", n);
}
use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> i32 {
    return get_input(io).trim().parse::<i32>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let ab : Vec<i32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let cd : Vec<i32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = get_input_int(&stdin);

    let a = (ab[0] - cd[0]).abs() + (ab[1] - cd[1]).abs();
    
    if n < a || (n - a) % 2 == 1 {
        println!("N");
    } else {
        println!("Y");
    }
}

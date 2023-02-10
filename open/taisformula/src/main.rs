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
    let n : u32 = get_input_int(&stdin);
    
    let mut old : Vec<f64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
    let mut sum : f64 = 0.0;
    
    for _i in 0..n - 1 {
        let new : Vec<f64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

        sum += (old[1] + new[1]) / 2.0 * (new[0] - old[0]) / 1000.0;
        old = new;
    }

    println!("{}", sum);
}
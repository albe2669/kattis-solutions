use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let m = n[2];
    let mut remainder = (n[0] + n[1]) % m;
    let mut posession = (n[0] + n[1] - remainder) / m;
    let mut total = posession;
    posession += remainder;

    while posession >= m {
        remainder = posession % m;
        posession = (posession - remainder) / m;
        total += posession;
        posession += remainder;
    }

    println!("{}", total)
}

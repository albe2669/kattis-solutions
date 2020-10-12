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
    let w : u32 = get_input_int(&stdin);
    let n : u32 = get_input_int(&stdin);

    let mut area : u32 = 0;

    for _i in 0..n {
        let t : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

        area += t[0] * t[1];
    }

    println!("{}", area/w);
}
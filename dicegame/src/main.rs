use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let gun : u32 = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<u32>>().iter().sum();
    let emm : u32 = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<u32>>().iter().sum();

    if gun > emm {
        println!("Gunnar");
    } else if emm > gun {
        println!("Emma");
    } else {
        println!("Tie");
    }
}
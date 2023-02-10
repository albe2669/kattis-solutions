use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    println!("{}", get_input(&stdin).replace("apa", "a").replace("epe", "e").replace("ipi", "i").replace("opo", "o").replace("upu", "u"));
}


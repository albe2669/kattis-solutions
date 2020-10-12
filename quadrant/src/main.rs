use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> i64 {
    return get_input(io).trim().parse::<i64>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let x : i64 = get_input_int(&stdin);
    let y : i64 = get_input_int(&stdin);

    if x > 0 && y > 0 {
        println!("1");
    } else if x < 0 && y > 0 {
        println!("2");
    } else if x < 0 && y < 0 {
        println!("3");
    } else if x > 0 && y < 0 {
        println!("4");
    }
}


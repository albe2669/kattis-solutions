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
    let n = get_input_int(&stdin);
    
    //let mut nums: Vec<i64> = Vec::new();
    for _i in 0..n {
        let t = get_input_int(&stdin);

        println!("{}", (t as f64 / 400f64).ceil());
    }
}

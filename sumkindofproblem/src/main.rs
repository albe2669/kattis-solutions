use std::io::{self};

fn get_input_int(io : &std::io::Stdin) -> i64 {
    return get_input(io).trim().parse().unwrap();
}

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let n : usize = get_input_int(&stdin) as usize;

    for _i in 0..n {
        let t : Vec<u64> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
        let num = t[1];

        println!(
            "{} {} {} {}", 
            t[0],
            (num  * (num + 1) / 2),
            (num * num),
            (num * (num + 1))
        )
    }
}
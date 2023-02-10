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
    let n : i64 = get_input_int(&stdin);

    let mut a : u32 = 1;
    let mut b : u32 = 0;

    // Just a simple sequence of fibonnaci numbers, where a is the current number, and b is the number just before it
    for _i in 0..n {
        let t = b.clone();

        b = a;
        a += t;
    }

    println!("{} {}", a - b, b);
}
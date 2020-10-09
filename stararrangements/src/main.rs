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
    let s = get_input_int(&stdin);

    println!("{}:", s);

    let mut f : i64 = 2;

    while f <= (s/2) + 1 {
        let rem : i64 = s % (2 * f - 1);
        if rem == 0 || rem - f == 0 {
            println!("{},{}", f, f - 1);
        }

        let rem : i64 = s % f;
        if rem == 0 {
            println!("{},{}", f, f);
        }

        f += 1;
    }
}
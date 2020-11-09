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

    let mut n : i64 = get_input_int(&stdin);
    let mut i = 1;

    while n != 0 {
        println!("SET {}", i);
        let mut v : Vec<String> = vec!["".to_owned(); n as usize];

        for j in 0..(n / 2) as usize {
            v[j] = get_input(&stdin).trim().to_owned();
            v[(n - 1) as usize - j] = get_input(&stdin).trim().to_owned();
        }

        if n % 2 != 0 {
            v[(n / 2) as usize] = get_input(&stdin).trim().to_owned();
        }

        for string in v {
            println!("{}", string);
        }

        i += 1;
        n = get_input_int(&stdin);
    }
}
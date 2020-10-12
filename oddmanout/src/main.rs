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
    let n : u32 = get_input_int(&stdin);

    for i in 0..n {
        get_input_int(&stdin);
        let guests : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();

        for guest in guests.iter() {
            if guests.iter().filter(|&n| *n == *guest).count() < 2 {
                println!("Case #{}: {}", i+1, guest);
            }
        }
    }
}


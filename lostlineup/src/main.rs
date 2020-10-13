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

    let people : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
    let mut answer : Vec<u32> = vec![0; n as usize];
    let mut t = 2;

    for i in people {
        answer[(i + 1) as usize] = t;
        t += 1;
    }

    answer[0] = 1;

    for i in answer {
        print!("{} ", i);
    }
    println!("");
}
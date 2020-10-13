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

    for _i in 0..n  {
        let s : String = get_input(&stdin).trim().to_string();
        if !s.contains('+') {
            println!("skipped");
        } else {
            let nums : Vec<u32> = s.clone().splitn(2, '+').map(|x| x.parse::<u32>().unwrap()).collect();
            println!("{}", nums[0] + nums[1]);
        }
    }
}


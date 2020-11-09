use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let mut i : Vec<char> = get_input(&stdin).trim().chars().collect();

    let mut one : Vec<char> = i.split_off(i.len() / 3);
    let two : Vec<char> = one.split_off(one.len() / 2);

    if one == two {
        println!("{}", one.iter().collect::<String>());
    } else if one == i {
        println!("{}", one.iter().collect::<String>());
    } else if two == i {
        println!("{}", two.iter().collect::<String>());
    }
}
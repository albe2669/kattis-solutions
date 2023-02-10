use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let s : Vec<char> = get_input(&stdin).trim().chars().collect();

    let mut o : String = "".to_owned();
    let mut curr : char = s[0];

    for i in 0..s.len() {
        if i + 1 < s.len() {
            if s[i + 1] != curr {
                o.push(curr);
                curr = s[i + 1];
            }
        } else {
            o.push(curr);
        }
    }

    println!("{}", o);
}
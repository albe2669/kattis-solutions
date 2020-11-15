use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let mut n : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut t : bool = true;
    while t {
        t = false;
        if n[0] > n[1] {
            n.swap(0, 1);
            t = true;
            println!("{}", n.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
        }

        if n[1] > n[2] {
            n.swap(1,2);
            t = true;
            println!("{}", n.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
        }

        if n[2] > n[3] {
            n.swap(2,3);
            t = true;
            println!("{}", n.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
        }

        if n[3] > n[4] {
            n.swap(3,4);
            t = true;
            println!("{}", n.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
        }
        
    }
}

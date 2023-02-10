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

    let ascii_lower : Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
    ];

    for _i in 0..n {
        let t : Vec<char> = get_input(&stdin).trim().to_ascii_lowercase().chars().collect();
        let mut del : Vec<char> = ascii_lower.clone();

        for letter in t.iter() {
            if del.contains(&letter) {
                del.retain(|&x| x != *letter);
            }
        }

        if del.len() == 0 {
            println!("pangram");
        } else {
            print!("missing ");
            for letter in del.iter() {
                print!("{}", letter);
            }
            println!("");
        }
    }
}


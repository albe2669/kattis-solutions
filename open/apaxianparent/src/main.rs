use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let s : Vec<String> = get_input(&stdin).trim().split_whitespace().map(|s| s.to_string()).collect();
    let mut firstname : Vec<char> = s[0].chars().collect();

    let last_char : char = firstname[firstname.len() - 1];

    if last_char == 'e' {
        println!("{}x{}", firstname.iter().collect::<String>(), s[1]);        
    } else if last_char == 'a' || last_char == 'i' || last_char == 'o' || last_char == 'u' {
        firstname.remove(firstname.len() - 1);
        println!("{}ex{}", firstname.iter().collect::<String>(), s[1]);        
    } else if last_char == 'x' && firstname[firstname.len() - 2] == 'e' {
        println!("{}{}", s[0], s[1]);        
    } else {
        println!("{}ex{}", s[0], s[1]);        
    }
}


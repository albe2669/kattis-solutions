use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let s = get_input(&stdin).trim().to_owned();
    let p = get_input(&stdin).trim().to_owned();

    if p == s {
        println!("Yes");

        return;
    }

    for i in 0..10 {
        let mut new = p.clone();
        new.insert_str(0, &i.to_string());

        if new == s {
            println!("Yes");

            return;
        }
    }
    
    for i in 0..10 {
        let mut new = p.clone();
        new.insert_str(new.len(), &i.to_string());
        
        if new == s {
            println!("Yes");

            return;
        }
    }
    
    let mut new : Vec<char> = p.chars().collect();

    for i in 0..new.len() {
        if new[i].is_ascii_lowercase() {
            new[i] = new[i].to_ascii_uppercase();
        } else if new[i].is_ascii_uppercase() {
            new[i] = new[i].to_ascii_lowercase()
        }
    }

    if new.iter().collect::<String>() == s {
        println!("Yes");

        return;
    }

    println!("No");
}

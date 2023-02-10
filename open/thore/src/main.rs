use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> i64 {
    return get_input(io).trim().parse::<i64>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let n = get_input_int(&stdin);
    let mut scores : Vec<String> = Vec::new();

    for _i in 0..n {
        scores.push(get_input(&stdin).trim().to_owned());
    }

    let the_thore : usize = scores.iter().position(|s| s == "ThoreHusfeldt").unwrap();

    if the_thore == 0 {
        println!("Thore is awesome");

        return;
    } 

    let tho : Vec<char> = "ThoreHusfeldt".chars().collect();
    let mut prefix : Vec<char> = Vec::new();
    for i in 0..the_thore {
        let score : String = scores[i].to_owned();
        
        if score.starts_with("ThoreHusfeld") {
            println!("Thore sucks");
            
            return;
        }

        let mut tmp : Vec<char> = Vec::new();
        let s : Vec<char> = score.chars().collect();
        
        for i in 0..(tho.len().min(s.len())) {
            if tho[i] == s[i] {
                tmp.push(tho[i]);
            } else {
                break;
            }
        }

        if tmp.len() >= prefix.len() {
            tmp.push(tho[tmp.len()]);

            prefix = tmp;             
        }

        
    }

    println!("{}", prefix.iter().collect::<String>());
}

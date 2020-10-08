use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let l1 : Vec<u32> = get_input(&stdin).split_whitespace().map(|num| num.parse().unwrap()).collect();
    
    let safe : u32 = l1[0];
    let mut current : u32 = 0;
    let mut disallowed : u32 = 0;

    for _i in 0..l1[1] {
        let mut iter : Vec<String> = get_input(&stdin).split_whitespace().map(|s| s.to_string()).collect();

        let keyword : String = iter.remove(0);
        let change : u32 = iter.remove(0).parse().unwrap();
    
        if keyword == "enter" {
            if change + current > safe {
                disallowed += 1;
                continue;
            }

            current += change;
        } else if keyword == "leave" {
            current -= change;
        }
    }

    println!("{}", disallowed);
}
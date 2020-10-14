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
    let p : u32 = get_input_int(&stdin);

    for _i in 0..p {
        let tmp : Vec<String> = get_input(&stdin).trim().split_whitespace().map(|s| s.to_string()).collect();

        let pq : Vec<u32> = tmp[1].split('/').map(|s| s.parse().unwrap()).collect();
        let mut en : u32 = pq[0];
        let mut de : u32 = pq[1];
        let mut sequence = String::from("");

        while en > 1 || de > 1 {
            if de > en {
                de -= en;
                sequence.push('L');
            } else {
                en -= de;
                sequence.push('R');
            }
        }

        let mut i : u32 = 1;
        let mut sequence : Vec<char> = sequence.chars().collect();
        sequence.reverse();
        
        for s in sequence {
            i *= 2;

            if s == 'R' {
                i += 1;
            }
        }

        println!("{} {}", tmp[0], i);
    }
}
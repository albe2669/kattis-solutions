use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let rc : Vec<u32> = get_input(&stdin).split_whitespace().map(|n| n.parse().unwrap()).collect();
    let mut g : Vec<Vec<char>> = Vec::new();
    let mut v : Vec<u32> = Vec::new();
    v.resize(5, 0);

    for _ in 0..rc[1] {
        g.push(get_input(&stdin).chars().collect());
    }

    for i in 0..(rc[1] as usize)-1 {
        for j in 0..(rc[0] as usize)-1 {
            let a : char = g[j]  [i];
            let b : char = g[j+1][i];
            let c : char = g[j]  [i+1];
            let d : char = g[j+1][i+1];

            if a == '#' || b == '#' || c == '#' || d == '#' {
                continue;
            }

            let mut n : u32 = 0;
            if a == 'X' {
                n += 1;
            }
            if b == 'X' {
                n += 1;
            }
            if c == 'X' {
                n += 1;
            }
            if d == 'X' {
                n += 1;
            }

            v[n as usize] += 1;
        }
    }

    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}
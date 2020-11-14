use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let dict : HashMap<char, u32> = [
        ('B', 1),
        ('F', 1),
        ('P', 1),
        ('V', 1),
        ('C', 2),
        ('G', 2),
        ('J', 2),
        ('K', 2),
        ('Q', 2),
        ('S', 2),
        ('X', 2),
        ('Z', 2),
        ('D', 3),
        ('T', 3),
        ('L', 4),
        ('M', 5),
        ('N', 5),
        ('R', 6),
    ].iter().cloned().collect();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut out : Vec<u32> = Vec::new();
        let mut last : u32 = 0;

        for c in line.trim().chars().collect::<Vec<char>>() {
            let i : u32 = match dict.get(&c) {
                Some(t) => t.to_owned(),
                None => {
                    last = 0;
                    continue;
                }
            };
            
            if i != last {
                out.push(i);
                last = i;
            }
        } 

        println!("{}", out.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
    }
}

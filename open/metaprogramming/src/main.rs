use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut register : HashMap<String, i32> = HashMap::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let command : Vec<String> = line.trim().split_whitespace().map(|s| s.to_owned()).collect();

        if command[0] == "define" {
            register.insert(command[2].clone(), command[1].parse::<i32>().unwrap());
        } else {
            let var1 = match register.get(&command[1]) {
                Some(v) => v,
                None => {
                    println!("undefined");
                    continue;
                }
            };
            
            let var2 = match register.get(&command[3]) {
                Some(v) => v,
                None => {
                    println!("undefined");
                    continue;
                }
            };

            match command[2].as_str() {
                "<" => println!("{}", var1 < var2),

                ">" => println!("{}", var1 > var2),
                "=" => println!("{}", var1 == var2),
                _ => ()
            }
        }
    }
}

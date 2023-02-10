use std::io::{self};
use std::collections::HashMap;

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let mut problems : HashMap<String, Vec<i64>> = HashMap::new();


    loop {
        let i : Vec<String> = get_input(&stdin).trim().split_whitespace().map(|s| s.to_owned()).collect();

        if i[0] == "-1" {
            break;
        }

        if problems.contains_key(&i[1]) {
            let problem : &mut Vec<i64> = problems.get_mut(&i[1]).unwrap();

            if i[2] == "right" {
                problem[0] += i[0].parse::<i64>().unwrap();
                problem[1] = 1;
            } else {
                problem[0] += 20;
            }
        } else {
            if i[2] == "right" {
                problems.insert(i[1].clone(), vec![i[0].parse::<i64>().unwrap(), 1]);
            } else {
                problems.insert(i[1].clone(), vec![20, 0]);
            }
        }
    }

    let mut sum = 0;
    let mut solved = 0; 
    for (_problem, result) in problems {
        if result[1] == 1 {
            sum += result[0];
            solved += 1;
        }
    }

    println!("{} {}", solved, sum);
}
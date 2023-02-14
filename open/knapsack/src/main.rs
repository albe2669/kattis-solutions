use std::collections::HashMap;
use std::io;

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn try_get_input(io: &std::io::Stdin) -> Option<String> {
    let mut buffer = String::new();
    match io.read_line(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

fn solve(objects: Vec<(u64, u64)>, capacity: u64) {
    let mut solutions: HashMap<u64, (u64, Vec<String>)> = HashMap::new();
    solutions.insert(0, (0, Vec::new()));

    for (index, (obj_value, obj_weight)) in objects.iter().enumerate() {
        for (weight, (value, indexes)) in solutions.clone().iter() {
            let new_weight = weight + obj_weight;
            if new_weight > capacity {
                continue;
            }

            let new_value = value + obj_value;
            let existing = solutions
                .get(&new_weight)
                .unwrap_or(&(0, Vec::new()))
                .to_owned();

            if existing.0 < new_value {
                let mut new_indexes = indexes.clone();
                new_indexes.push(index.to_string());
                solutions.insert(new_weight, (new_value, new_indexes));
            }
        }
    }

    let mut max_value = 0;
    let mut max_indexes = Vec::new();
    for (_, (value, indexes)) in solutions.iter() {
        if *value > max_value {
            max_value = *value;
            max_indexes = indexes.clone();
        }
    }

    println!("{}", max_indexes.len());
    println!("{}", max_indexes.join(" "));
}

fn main() {
    let stdin = io::stdin();

    while let Some(cn) = try_get_input(&stdin) {
        if cn.trim() == "" {
            break;
        }
        let cn = cn
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let capacity = cn[0];
        let n = cn[1] as usize;

        let mut objects = Vec::new();
        for _ in 0..n {
            let obj: Vec<u64> = get_input(&stdin)
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            objects.push((obj[0], obj[1]));
        }

        solve(objects, capacity);
    }
}

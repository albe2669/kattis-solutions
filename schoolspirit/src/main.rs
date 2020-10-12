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
    let n : u32 = get_input_int(&stdin);
    let mut sum : f64 = 0f64;
    let mut students : Vec<f64> = Vec::new();

    for i in 0..n {
        let t : f64 = get_input_int(&stdin) as f64;

        students.push(t);

        sum += t* (4f64/5f64).powf(i as f64);
    }

    let mut g : Vec<f64> = vec![0.0; students.len()];

    for i in 0..g.len() {
        let mut new_students : Vec<f64> = students.clone();
        new_students.remove(i);

        for s in 0..new_students.len() {
            g[i] += new_students[s] * (4f64/5f64).powf(s as f64);
        }

        g[i] = 1f64/5f64 * g[i];
    }
    

    println!("{:.14}", 1f64/5f64 * sum);
    println!("{:.14}", g.iter().sum::<f64>() / (g.len() as f64));
}
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
    let mut n : i64 = get_input_int(&stdin);

    while n != -1 {
        let mut vertices : Vec<Vec<u32>> = Vec::new();

        for _i in 0..n as usize {
            vertices.push(get_input(&stdin).split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u32>>());
        }

        for i in 0..n as usize {
            let mut weak : bool = true;

            for j in 0..n as usize {
                for k in 0..n as usize {
                    if  vertices[i][k] == 1 &&
                        vertices[i][j] == 1 &&
                        vertices[j][k] == 1 &&
                        i != k &&
                        j != k {
                            weak = false;
                        }
                }
            }

            if weak {
                print!("{} ", i);
            }
        }

        println!("");

        n = get_input_int(&stdin);
    }
}
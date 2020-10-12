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
    let t : u32 = get_input_int(&stdin);

    for _i in 0..t {
        let n : Vec<char> = get_input(&stdin).trim().chars().collect::<Vec<char>>();
        let mut sum : u32 = 0;

        for c in (0..n.len()).rev() {
            let int : u32 = (n[c].to_string()).parse::<u32>().unwrap();
            let mut new_int : u32 = int;

            if c % 2 == 0 {
                new_int *= 2;

                if new_int > 9 {
                    let tmp : Vec<char> = new_int.to_string().chars().collect();

                    new_int = 0;
                    for j in tmp {
                        new_int += (j.to_string()).parse::<u32>().unwrap();
                    }
                }
            }

            println!("{} - {} = {}", c, int, new_int);
            sum += new_int;
        }

        println!("{}", if sum % 10 == 0 { "PASS" } else { "FAIL" });
    }
} // Not complete
use std::io;

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn search(fruit: u64, n: u64, fruits: &Vec<u64>, sum: u64, res: &mut u64) {
    if sum >= 200 {
        *res += sum;
        return;
    } else if fruit == n {
        return;
    }

    for i in 0..n {
        search(i + 1, n, fruits, sum + fruits[fruit as usize], res);
    }
}

fn main() {
    let stdin = io::stdin();

    let n: u64 = get_input(&stdin).trim().parse().unwrap();

    let fruits: Vec<u64> = get_input(&stdin)
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut res = 0;
    search(0, n, &fruits, 0, &mut res);
    println!("{}", res);
}

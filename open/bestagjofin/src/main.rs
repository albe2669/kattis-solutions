use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: usize = input.next().unwrap().trim().parse().unwrap();

    let mut best: u32 = 0;
    let mut best_person = "";

    for _ in 0..n {
        let person = input.next().unwrap().trim();
        let x: u32 = input.next().unwrap().trim().parse().unwrap();

        if x > best {
            best = x;
            best_person = person;
        }
    }

    output.write_fmt(format_args!("{}\n", best_person)).unwrap();
}

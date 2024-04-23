use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: f64 = input.next().unwrap().trim().parse().unwrap();

    let l1 = (n / 7f64) * 3f64;
    let l2 = (l1 / 4f64) * 6f64;

    writeln!(output, "{}", (l1 * l1 * 2f64 + l1 * l2 * 2f64) as i64).unwrap();
}

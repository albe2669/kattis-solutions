use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let numbers = input
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let v = numbers[0];
    let a = numbers[1];
    let t = numbers[2];

    output
        .write_fmt(format_args!("{:.9}\n", v * t + 0.5f64 * a * t.powf(2f64)))
        .unwrap();
}

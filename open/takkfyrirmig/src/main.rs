use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: usize = input.next().unwrap().trim().parse().unwrap();

    for i in 0..n {
        let person = input.next().unwrap().trim();

        output.write_fmt(format_args!("Takk {}\n", person)).unwrap();
    }
}

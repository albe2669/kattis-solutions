use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let s = input.next().unwrap().trim();

    output
        .write_fmt(format_args!("{} {} {}\n", s, s, s))
        .unwrap();
}

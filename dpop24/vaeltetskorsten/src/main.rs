use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: u32 = input.next().unwrap().trim().parse().unwrap();

    let mut res = 0u32;

    for i in 0..n {
        let x: u32 = input.next().unwrap().trim().parse().unwrap();
        let s = input.next().unwrap().trim();

        if s == "nej" {
            res = x.max(res);
        }
    }

    output.write_fmt(format_args!("{}\n", res)).unwrap();
}

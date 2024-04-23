use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace().next().unwrap().trim().split('-');

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let a = input.next().unwrap().parse::<i32>().unwrap();
    let b = input.next().unwrap().parse::<i32>().unwrap();

    let res = match (a, b, (a - b).abs()) {
        (30, 30, _) => "!",
        (30, _, diff) => {
            if diff > 2 {
                "!"
            } else {
                "A"
            }
        }
        (_, 30, diff) => {
            if diff > 2 {
                "!"
            } else {
                "B"
            }
        }
        (a, b, diff) if diff > 2 && (a > 21 || b > 21) => "!",
        (a, b, diff) if diff >= 2 && (a >= 21 || b >= 21) && a > b => "A",
        (a, b, diff) if diff >= 2 && (a >= 21 || b >= 21) && a < b => "B",
        _ => "?",
    };

    output.write_fmt(format_args!("{}\n", res)).unwrap();
}

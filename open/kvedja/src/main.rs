use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    output
        .write_fmt(format_args!("Kvedja,\n{}\n", buffer))
        .unwrap();
}

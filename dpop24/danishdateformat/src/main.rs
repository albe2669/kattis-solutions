use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace().next().unwrap().trim().split('/');

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let month = match input.next().unwrap() {
        "01" => "januar",
        "02" => "februar",
        "03" => "marts",
        "04" => "april",
        "05" => "maj",
        "06" => "juni",
        "07" => "juli",
        "08" => "august",
        "09" => "september",
        "10" => "oktober",
        "11" => "november",
        "12" => "december",
        _ => "you fucked up",
    };

    output
        .write_fmt(format_args!(
            "{}. {} {}",
            input.next().unwrap().parse::<i64>().unwrap(),
            month,
            input.next().unwrap()
        ))
        .unwrap();
}

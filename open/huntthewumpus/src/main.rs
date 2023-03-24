use std::{
    collections::HashSet,
    io::{self, Read, Write},
};

fn get_last_digits(number: i64) -> (i64, i64) {
    let string = number.to_string().chars().collect::<Vec<char>>();
    let last = string[string.len() - 1];
    let second_last = string[string.len() - 2];

    (
        second_last.to_digit(10).unwrap() as i64,
        last.to_digit(10).unwrap() as i64,
    )
}

fn get_random(s_prev: i64) -> (i64, (i64, i64)) {
    let s = s_prev + (s_prev as f64 / 13f64).floor() as i64 + 15;
    (s, get_last_digits(s))
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let mut s: i64 = input.next().unwrap().trim().parse().unwrap();
    let mut locations = HashSet::new();
    while locations.len() != 4 {
        let calc = get_random(s);
        s = calc.0;

        if locations.contains(&calc.1) {
            continue;
        }

        locations.insert(calc.1);
    }

    let mut moves = 0;

    for guess in input {
        moves += 1;

        let mut chars = guess.chars();
        let guess = (
            chars.next().unwrap().to_digit(10).unwrap() as i64,
            chars.next().unwrap().to_digit(10).unwrap() as i64,
        );
        if locations.contains(&guess) {
            output.write_all(b"You hit a wumpus!\n").unwrap();
            locations.remove(&guess);

            if locations.is_empty() {
                break;
            }
        }

        let mut min = i64::MAX;
        for loc in locations.iter() {
            let distance = (loc.0 - guess.0).abs() + (loc.1 - guess.1).abs();
            if distance < min {
                min = distance;
            }
        }

        output.write_fmt(format_args!("{}\n", min)).unwrap();
    }

    output
        .write_fmt(format_args!("Your score is {} moves.\n", moves))
        .unwrap();
}

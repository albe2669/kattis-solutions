use std::io::{self, Read, Write};

enum Game {
    A,
    B,
    Incomplete,
    Invalid,
}

fn resolve_game(s: &str) -> Game {
    let mut input = s.trim().split('-');

    let a = input.next().unwrap().parse::<i32>().unwrap();
    let b = input.next().unwrap().parse::<i32>().unwrap();

    match (a, b, (a - b).abs()) {
        (30, 30, _) => Game::Invalid,
        (30, _, diff) => {
            if diff > 2 {
                Game::Invalid
            } else {
                Game::A
            }
        }
        (_, 30, diff) => {
            if diff > 2 {
                Game::Invalid
            } else {
                Game::B
            }
        }
        (a, b, diff) if diff > 2 && (a > 21 || b > 21) => Game::Invalid,
        (a, b, diff) if diff >= 2 && (a >= 21 || b >= 21) && a > b => Game::A,
        (a, b, diff) if diff >= 2 && (a >= 21 || b >= 21) && a < b => Game::B,
        _ => Game::Incomplete,
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let mut result = vec![0, 0, 0];
    let mut matches = 0;

    let mut prev_game: Game = Game::A;

    for set in input {
        if matches!(prev_game, Game::Incomplete) {
            output.write_fmt(format_args!("!\n")).unwrap();
            return;
        }

        matches += 1;
        if matches > 3 {
            output.write_fmt(format_args!("!\n")).unwrap();
            return;
        }

        let game = resolve_game(set);
        match game {
            Game::A => {
                result[0] += 1;
            }
            Game::B => {
                result[1] += 1;
            }
            Game::Invalid => {
                output.write_fmt(format_args!("!\n")).unwrap();
                return;
            }
            _ => {}
        }

        prev_game = game;
    }

    let res = match (result[0], result[1], prev_game) {
        (2, _, Game::Incomplete) => "!",
        (_, 2, Game::Incomplete) => "!",
        (2, _, Game::A) => "A",
        (_, 2, Game::B) => "B",
        _ => "?",
    };

    output.write_fmt(format_args!("{}\n", res)).unwrap();
}

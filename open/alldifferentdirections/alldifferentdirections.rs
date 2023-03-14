use std::{
    io,
    ops::{Add, Div, Mul, Sub},
};

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    fn abs_square(self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }

    fn recip(self) -> Self {
        let denominator = self.abs_square();
        Self::new(self.real / denominator, -self.imaginary / denominator)
    }

    fn abs(self) -> f64 {
        self.abs_square().sqrt()
    }
}

impl From<f64> for Complex {
    fn from(f: f64) -> Self {
        Self::new(f, 0.0)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imaginary + other.imaginary)
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.real - other.real, self.imaginary - other.imaginary)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.real * other.real - self.imaginary * other.imaginary,
            self.real * other.imaginary + self.imaginary * other.real,
        )
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        let n: usize = get_input(&stdin).trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut positions = Vec::with_capacity(n);

        for _ in 0..n {
            let line = get_input(&stdin);
            let mut iter = line.split_whitespace();

            let mut position = Complex::new(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            );

            iter.next(); // skip "start"
            let mut direction: f64 = iter.next().unwrap().parse().unwrap();

            while let Some(operation) = iter.next() {
                let value = iter.next().unwrap().parse().unwrap();

                if operation == "walk" {
                    position = position
                        + Complex::from_polar(value, direction * std::f64::consts::PI / 180.0);
                } else {
                    direction += value;
                }
            }

            positions.push(position);
        }

        let mut avg = Complex::new(0.0, 0.0);
        for position in positions.iter() {
            avg = avg + *position;
        }
        avg = avg / Complex::from(n as f64);

        let mut maximum_distance = 0.0;
        for position in positions.iter() {
            let distance = (*position - avg).abs();
            if distance > maximum_distance {
                maximum_distance = distance;
            }
        }

        println!(
            "{:.5} {:.5} {:.5}",
            avg.real, avg.imaginary, maximum_distance
        );
    }
}

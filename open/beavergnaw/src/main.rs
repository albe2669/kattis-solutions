use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let pi : f64 = std::f64::consts::PI;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<f64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let d = nums[0];
        let v = nums[1];

        if d == 0.0 && v == 0.0 {
            break;
        }

        println!("{:.9}", (((((pi * (d/2.0).powf(2.0) * d - v) - (pi * (d/2.0).powf(2.0) * d/3.0)) * 1.5)/(2.0 * pi)).powf(1.0/3.0)) * 2.0);
    }
}
use std::io::{self, Read, Write};

fn calc_distance(x: usize, y: usize, grid: &Vec<Vec<u32>>, curr_min: u64) -> Option<u64> {
    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if sum > curr_min {
                return None;
            }

            let distance = (x as i32 - i as i32).abs() + (y as i32 - j as i32).abs();
            sum += grid[i][j] as u64 * distance as u64;
        }
    }

    Some(sum)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: usize = input.next().unwrap().trim().parse().unwrap();
    let m: usize = input.next().unwrap().trim().parse().unwrap();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; m]; n];

    (0..n).for_each(|i| {
        for j in 0..m {
            grid[i][j] = input.next().unwrap().trim().parse().unwrap();
        }
    });

    let mut min_sum = std::u64::MAX;
    for i in 0..n {
        for j in 0..m {
            if let Some(sum) = calc_distance(i, j, &grid, min_sum) {
                if sum < min_sum {
                    min_sum = sum;
                }
            }
        }
    }

    output.write_fmt(format_args!("{}", min_sum)).unwrap()
}

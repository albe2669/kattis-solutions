use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut x_max = -10f64;
    let mut x_min = 20f64;
    let mut y_max = -10f64;
    let mut y_min = 20f64;
    let mut circles = Vec::with_capacity(n);

    for line in lines {
        let mut nums = line.split_whitespace().map(|n| n.parse::<f64>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        let r = nums.next().unwrap();
        circles.push((x, y, r));

        if x + r > x_max {
            x_max = x + r;
        }
        if x - r < x_min {
            x_min = x - r;
        }
        if y + r > y_max {
            y_max = y + r;
        }
        if y - r < y_min {
            y_min = y - r;
        }
    }

    let points = 500f64;
    let x_step = (x_max - x_min) / points;
    let y_step = (y_max - y_min) / points;

    let mut count = 0;

    for i in 0..(points as i32) {
        let x = x_min + x_step * (i as f64);
        for j in 0..(points as i32) {
            let y = y_min + y_step * (j as f64);
            for &(cx, cy, r) in circles.iter() {
                let dx = (x - cx).powi(2);
                let dy = (y - cy).powi(2);
                if dx + dy <= r.powi(2) {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("{:.10}", (count as f64) * x_step * y_step);
}

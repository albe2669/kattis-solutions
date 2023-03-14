use std::io;

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn sort_min_angle(points: &[(f64, f64)], src: &(f64, f64)) -> Vec<(f64, f64)> {
    let mut sorted = points
        .iter()
        .map(|x| {
            (
                (x.1 - src.1).atan2(x.0 - src.0),
                (x.1 - src.1).hypot(x.0 - src.0),
                *x,
            )
        })
        .collect::<Vec<(f64, f64, (f64, f64))>>();

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sorted.into_iter().map(|x| x.2).collect()
}

fn calc_product(a: &(f64, f64), b: &(f64, f64), c: &(f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

fn create_convex(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    if points.is_empty() {
        return Vec::new();
    }

    let mut convex = Vec::new();
    let mut minimum = points
        .iter()
        .min_by(|a, b| {
            let ord = a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal);
            match ord {
                std::cmp::Ordering::Equal => {
                    a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                }
                _ => ord,
            }
        })
        .unwrap();

    let points = sort_min_angle(points, minimum);

    if points.len() <= 3 {
        return points;
    }

    for point in points {
        while convex.len() > 1
            && calc_product(&convex[convex.len() - 2], &convex[convex.len() - 1], &point) < 0f64
        {
            convex.pop();
        }

        convex.push(point);
    }

    convex
}

fn main() {
    let stdin = io::stdin();

    loop {
        let n: u32 = get_input(&stdin).trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut points = Vec::new();
        for _ in 0..n {
            let point: Vec<f64> = get_input(&stdin)
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            points.push((point[0], point[1]));
        }

        let convex = create_convex(&points);
        let mut sum = 0f64;
        let mut j = convex.len() - 1;

        for i in 0..convex.len() {
            sum += (convex[j].0 + convex[i].0) * (convex[j].1 - convex[i].1);
            j = i;
        }

        println!("{:.1}", (sum / 2f64).abs());
    }
}

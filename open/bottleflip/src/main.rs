use std::io;

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

struct Bottle {
    height: f64,
    air_density: f64,
    wat_density: f64,
}

impl Bottle {
    fn calc_com(&self, wat_height: f64) -> f64 {
        let weight = (wat_height * wat_height * self.wat_density
            + (self.height + wat_height) * (self.height - wat_height) * self.air_density)
            / 2f64;
        let mass = wat_height * self.wat_density + (self.height - wat_height) * self.air_density;

        weight / mass
    }
}

fn search(low: f64, high: f64, it: u32, bottle: Bottle) -> f64 {
    if it == 200 {
        return (high + low) / 2f64;
    }

    let mid1 = (2f64 * low + high) / 3f64;
    let mid2 = (low + 2f64 * high) / 3f64;

    let com1 = bottle.calc_com(mid1);
    let com2 = bottle.calc_com(mid2);

    // mid2 is too high
    if com1 > com2 {
        search(mid1, high, it + 1, bottle)
    } else if com1 < com2 {
        search(low, mid2, it + 1, bottle)
    } else {
        search(mid1, mid2, it + 1, bottle)
    }
}

fn main() {
    let stdin = io::stdin();

    let n: Vec<f64> = get_input(&stdin)
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let bottle = Bottle {
        height: n[0],
        air_density: n[2],
        wat_density: n[3],
    };

    println!("{:.10}", search(0f64, bottle.height / 2f64, 0, bottle));
}

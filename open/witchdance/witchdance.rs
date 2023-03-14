use std::{
    cmp::max,
    collections::HashMap,
    f64::consts::PI,
    io::{self, Read},
};

#[derive(Debug, Clone)]
struct Witch {
    x: f64,
    y: f64,
    r: f64,
}

impl Witch {
    fn collision(&self, other: &Witch) -> bool {
        let distance = (self.x - other.x).powi(2) + (self.y - other.y).powi(2);
        if distance <= 1f64 {
            return true;
        } else if distance > 4f64 {
            return false;
        }

        if f64::min((self.r - other.r - PI).abs(), (self.r - other.r + PI).abs())
            <= 2f64 * ((distance.sqrt() / 2.0).acos())
        {
            return true;
        }

        false
    }

    fn hash(&self) -> (i64, i64) {
        ((2f64 * self.x) as i64, (2f64 * self.y) as i64)
    }
}

fn read() -> Option<HashMap<(i64, i64), Witch>> {
    let mut buffer = String::new();
    let mut handle = io::stdin().lock();

    handle.read_to_string(&mut buffer).unwrap();

    let mut numbers = buffer.split_whitespace().map(|s| s.parse::<f64>().unwrap());

    let n = numbers.next().unwrap() as usize;
    let mut witches = HashMap::with_capacity(n);

    for _ in 0..n {
        let w = Witch {
            x: numbers.next().unwrap(),
            y: numbers.next().unwrap(),
            r: numbers.next().unwrap(),
        };
        let hash = w.hash();

        if witches.contains_key(&hash) {
            return None;
        }

        witches.insert(hash, w);
    }

    Some(witches)
}

fn main() {
    let witches = match read() {
        Some(witches) => witches,
        None => {
            println!("crash");
            return;
        }
    };

    for ((a, b), witch) in witches.iter() {
        for dx in 0..5 {
            for dy in max(-4, 1 - 5 * dx)..5 {
                if let Some(other) = witches.get(&(a + dx, b + dy)) {
                    if witch.collision(other) {
                        println!("crash");
                        return;
                    }
                }
            }
        }
    }

    println!("ok");
}

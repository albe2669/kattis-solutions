use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    io,
};

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

type Graph = HashMap<String, HashSet<String>>;

fn dfs(edges: &mut Graph, source: String, target: String) -> Option<Vec<String>> {
    if source == target {
        return Some(vec![source]);
    }

    let mut queue = VecDeque::new();
    queue.push_back(source.clone());

    let mut visited = HashSet::new();
    let mut result = HashMap::new();

    while let Some(vertex) = queue.pop_front() {
        if visited.contains(&vertex) {
            continue;
        }

        if vertex == target {
            return Some(build_path(&result, source, target));
        }

        visited.insert(vertex.clone());
        if let Some(neighbors) = edges.get(&vertex) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    queue.push_back(neighbor.clone());
                    result.insert(neighbor.clone(), vertex.clone());
                }
            }
        }
    }

    None
}

fn build_path(result: &HashMap<String, String>, source: String, target: String) -> Vec<String> {
    let mut path = vec![target.clone()];
    let mut current = target;

    while current != source {
        current = result.get(&current).unwrap().clone();
        path.push(current.clone());
    }

    path.reverse();
    path
}

fn main() {
    let stdin = io::stdin();

    let mut edges: Graph = HashMap::new();

    let n = get_input(&stdin).trim().parse::<i32>().unwrap();

    for _ in 0..n {
        let line = get_input(&stdin);
        let mut line = line.split_whitespace();

        let station = line.next().unwrap().to_string();
        let mut set = HashSet::new();

        for target in line {
            match edges.entry(target.to_string()) {
                Entry::Vacant(v) => {
                    let mut set = HashSet::new();
                    set.insert(station.clone());
                    v.insert(set);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(station.clone());
                }
            }

            set.insert(target.to_string());
        }

        match edges.entry(station) {
            Entry::Vacant(v) => {
                v.insert(set);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().extend(set);
            }
        }
    }

    let line = get_input(&stdin);
    let mut line = line.split_whitespace();

    match dfs(
        &mut edges,
        line.next().unwrap().to_string(),
        line.next().unwrap().to_string(),
    ) {
        Some(path) => {
            println!("{}", path.join(" "));
        }
        None => println!("no route found"),
    }
}

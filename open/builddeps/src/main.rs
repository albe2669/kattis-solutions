use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    io,
};

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

type Graph = HashMap<String, Vec<String>>;

fn dfs(nodes: &mut Graph, source: String) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    let mut result = Vec::new();

    queue.push((source, true));

    while let Some((node, is_new)) = queue.pop() {
        if !is_new {
            result.push(node);
            continue;
        }

        if visited.contains(&node) {
            continue;
        }

        queue.push((node.clone(), false));
        visited.insert(node.clone());
        let deps = nodes.get(&node);
        if deps.is_none() {
            continue;
        }

        queue.append(
            deps.unwrap()
                .iter()
                .map(|x| (x.clone(), true))
                .collect::<Vec<_>>()
                .as_mut(),
        );
    }

    result
}

fn main() {
    let stdin = io::stdin();

    let n: usize = get_input(&stdin).trim().parse().unwrap();
    let mut nodes: Graph = HashMap::new();

    for _ in 0..n {
        let line = get_input(&stdin);
        let mut line = line.split_whitespace();

        let s = line.next().unwrap().strip_suffix(':').unwrap();

        for target in line {
            match nodes.entry(target.to_string()) {
                Entry::Vacant(v) => {
                    v.insert(vec![s.to_string()]);
                }
                Entry::Occupied(mut o) => {
                    o.get_mut().push(s.to_string());
                }
            }
        }
    }

    let target = get_input(&stdin).trim().to_string();
    let mut result = dfs(&mut nodes, target);
    result.reverse();
    println!("{}", result.join("\n"));
}

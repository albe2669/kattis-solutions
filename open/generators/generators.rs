use std::io;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

type Vertex = u64;
type Edge = Vertex;
type Graph = BTreeMap<Vertex, BTreeMap<Vertex, Edge>>;

fn add_edge(graph: &mut Graph, v1: Vertex, v2: Vertex, e: Edge) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, e);
    graph.entry(v2).or_insert_with(BTreeMap::new).insert(v1, e);
}

fn prim(graph: &mut Graph, start: Vertex) -> u64 {
    // Instantiate MST 
    let mut mst = Graph::new();
    mst.insert(start, BTreeMap::new());

    // Priority queue
    let mut prio = BinaryHeap::new();

    let mut sum: u64 = 0;

    for (vertex, edge) in &graph[&start] {
        // Simulate a min heap, read more: https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#min-heap
        prio.push(Reverse((*edge, vertex, start)));
    }

    while let Some(Reverse((edge, destv, prevv))) = prio.pop() {
        if mst.contains_key(destv) {
            continue;
        }

        add_edge(&mut mst, prevv, *destv, edge);
        sum += edge;

        for (vertex, edge2) in &graph[destv] {
            if !mst.contains_key(vertex) {
                prio.push(Reverse((*edge2, vertex, *destv)));
            }
        }
    }

    sum
}

fn read_line(io : &std::io::Stdin) -> Vec<Vertex> {
    get_input(io)
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<Vertex>>()
}

fn main() {
    let stdin = io::stdin();

    let nm = read_line(&stdin);
    let mut graph = Graph::new();

    for _ in 0..nm[1] {
        let deal = read_line(&stdin);

        // Cities start at index 1, so we call power for 0
        add_edge(&mut graph, 0, deal[0], deal[1]);
    }

    let cities = read_line(&stdin);
    for (i, el) in cities.iter().enumerate() {
        let city: Vertex = (i as Vertex) + 1;
        let mut dest = city + 1;
        if dest > nm[0] {
            dest = 1;
        }

        add_edge(&mut graph, city, dest, *el);
    }

    println!("{}", prim(&mut graph, 0));
}

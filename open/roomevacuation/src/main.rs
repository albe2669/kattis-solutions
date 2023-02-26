use std::{collections::HashMap, io};

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

type Index = usize;

#[derive(Debug, Clone)]
struct Edge {
    flow: i32,
    capacity: i32,
    residual: Option<(Index, Index)>,
}

impl Edge {
    fn new(from: Index, to: Index, capacity: i32) -> Self {
        Self {
            flow: 0,
            capacity,
            residual: Some((to, from)),
        }
    }

    fn remaining_capacity(&self) -> i32 {
        self.capacity - self.flow
    }

    fn augment(&mut self, bottle_neck: i32) {
        self.flow += bottle_neck;
    }
}

type Node = u32;

pub struct Graph {
    edges: HashMap<Index, HashMap<Index, Edge>>,
    nodes: HashMap<Index, Node>,
    counter: Index,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashMap::new(),
            counter: 0,
        }
    }

    fn add_node(&mut self, node: Node) -> Index {
        let index = self.counter;
        self.nodes.insert(index, node);
        self.counter += 1;
        index
    }

    fn node_edges(&self, node: Index) -> Vec<(Index, Index, &Edge)> {
        match self.edges.get(&node) {
            Some(e) => e.iter().map(|(to, edge)| (node, *to, edge)).collect(),
            None => Vec::new(),
        }
    }

    fn add_edge(&mut self, from: Index, to: Index) {
        let a = Edge::new(from, to, 1);
        let mut list = self.edges.entry(from).or_insert_with(HashMap::new);
        list.insert(to, a);

        let b = Edge::new(to, from, 0);
        list = self.edges.entry(to).or_insert_with(HashMap::new);
        list.insert(from, b);
    }

    fn get_mut_edge(&mut self, from: Index, to: Index) -> &mut Edge {
        let list = self.edges.get_mut(&from).unwrap();
        list.get_mut(&to).unwrap()
    }

    fn augment_edge(&mut self, from: Index, to: Index, bottle_neck: i32) {
        let edge = self.get_mut_edge(from, to);
        edge.augment(bottle_neck);

        let (from, to) = edge.residual.unwrap();
        let edge = self.get_mut_edge(from, to);
        edge.flow -= bottle_neck;
    }
}

struct Solver<'a> {
    graph: &'a mut Graph,
    source: Index,
    sink: Index,
    visited: Vec<u32>,
    visited_token: u32,
}

impl<'a> Solver<'a> {
    fn new(graph: &'a mut Graph, source: Index, sink: Index) -> Self {
        let n = graph.nodes.len();
        Self {
            graph,
            source,
            sink,
            visited: vec![0; n],
            visited_token: 1,
        }
    }

    fn dfs(&mut self, node: Index, flow: i32) -> i32 {
        if node == self.sink {
            return flow;
        }

        self.visited[node] = self.visited_token;
        // Please don't even ask
        let edges = self
            .graph
            .node_edges(node)
            .iter()
            .map(|(from, to, edge)| (*from, *to, (**edge).clone()))
            .collect::<Vec<(Index, Index, Edge)>>();

        for (from, to, edge) in edges {
            let rcap = edge.remaining_capacity();
            if self.visited[to] != self.visited_token && rcap > 0 {
                let bottleneck = self.dfs(to, std::cmp::min(flow, rcap));

                if bottleneck > 0 {
                    self.graph.augment_edge(from, to, bottleneck);
                    return bottleneck;
                }
            }
        }

        0
    }

    fn solve(&mut self) -> i32 {
        let mut flow = 0;
        let mut f = -1;

        while f != 0 {
            f = self.dfs(self.source, i32::MAX);
            flow += f;
            self.visited_token += 1;
        }

        flow
    }
}

fn main() {
    let stdin = io::stdin();

    let nmt: Vec<u32> = get_input(&stdin)
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let n = nmt[0] as usize; // height
    let m = nmt[1] as usize; // width
    let t = nmt[2]; // time to deadline

    let mut grid = vec![vec![]; n];
    grid.iter_mut().for_each(|x| {
        *x = get_input(&stdin).trim().chars().collect::<Vec<char>>();
    });

    let mut nodes: Vec<Vec<Vec<Vec<Index>>>> = Vec::with_capacity(n);
    let mut graph = Graph::new();

    let source = graph.add_node(0);
    let sink = graph.add_node(1);

    let mut c = 2;
    for _ in 0..n {
        let mut row: Vec<Vec<Vec<Index>>> = Vec::with_capacity(m);
        for _ in 0..m {
            let mut col: Vec<Vec<Index>> = Vec::with_capacity(t as usize);
            for _ in 0..(t + 1) as usize {
                let mut time: Vec<Index> = Vec::with_capacity(2);
                for _ in 0..2 {
                    let index = graph.add_node(c);
                    time.push(index);
                    c += 1;
                }
                col.push(time);
            }
            row.push(col);
        }
        nodes.push(row);
    }

    let dir: Vec<i32> = vec![0, -1, 0, 1, 0];

    for r in 0..n {
        for c in 0..m {
            match grid[r][c] {
                '#' => continue,
                'E' => {
                    for i in 1..(t + 1) as usize {
                        graph.add_edge(nodes[r][c][i][0], nodes[r][c][i][1]);
                        graph.add_edge(nodes[r][c][i][1], sink);
                    }
                    continue;
                }
                'P' => {
                    graph.add_edge(source, nodes[r][c][0][0]);
                }
                _ => {}
            }

            for i in 0..t as usize {
                graph.add_edge(nodes[r][c][i][0], nodes[r][c][i][1]);
                for d in 0..4 {
                    let r2 = (r as i32) + dir[d];
                    let c2 = (c as i32) + dir[d + 1];

                    if r2 < 0 || r2 >= (n as i32) || c2 < 0 || c2 >= (m as i32) {
                        continue;
                    }

                    if grid[r2 as usize][c2 as usize] == '#' {
                        continue;
                    }

                    graph.add_edge(nodes[r][c][i][1], nodes[r2 as usize][c2 as usize][i + 1][0]);
                }

                graph.add_edge(nodes[r][c][i][1], nodes[r][c][i + 1][0]);
            }
        }
    }

    let mut solver = Solver::new(&mut graph, source, sink);
    let t = solver.solve();
    println!("{:?}", t);
}

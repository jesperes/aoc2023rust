use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use petgraph::{graph::UnGraph, stable_graph::NodeIndex};

use crate::Solver;

pub struct Solution;

type GraphType = UnGraph<String, String>;
type ResultType = usize;

struct Data {
    graph: GraphType,
    map: HashMap<String, NodeIndex>,
}

impl Data {
    fn new_from_input(input: &str) -> Self {
        let mut graph = GraphType::new_undirected();
        let mut map = HashMap::new();

        let lines = input
            .lines()
            .map(|line| {
                line.split(|c| ": ".contains(c))
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect_vec()
            })
            .collect_vec();

        // Create the nodes
        for line in &lines {
            for e in line {
                if !map.contains_key(e) {
                    let idx = graph.add_node(e.to_string());
                    map.insert(e.to_string(), idx);
                }
            }
        }

        // Add the edges
        for line in &lines {
            let lhs = map.get(&line[0]).unwrap();
            for e in &line[1..] {
                let rhs = map.get(e).unwrap();
                graph.add_edge(*lhs, *rhs, String::new());
            }
        }
        Data { graph, map }
    }

    fn remove_edge(&mut self, a: &str, b: &str) {
        let a_idx = self.map.get(&a.to_string()).unwrap();
        let b_idx = self.map.get(&b.to_string()).unwrap();
        let edge_idx = self.graph.find_edge(*a_idx, *b_idx).unwrap();
        self.graph.remove_edge(edge_idx);
    }
}

impl Solver<ResultType, ResultType> for Solution {
    fn solve(&self, input: &str) -> (ResultType, ResultType) {
        solve(input)
    }
}

fn solve(input: &str) -> (ResultType, ResultType) {
    let mut data = Data::new_from_input(input);
    let p1 = solve_p1(&mut data);
    (p1, 0)
}

fn solve_p1(data: &mut Data) -> usize {
    // These were found by visual inspection of the graph (plotted using
    // Dot::new)

    data.remove_edge("rxt", "bqq");
    data.remove_edge("qxr", "btp");
    data.remove_edge("vfx", "bgl");

    let total_size = data.graph.node_count();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(data.graph.node_indices().next().unwrap());

    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        for nbr in data.graph.neighbors(node) {
            queue.push_back(nbr);
        }
    }

    let size_set1 = visited.len();
    size_set1 * (total_size - size_set1)
}

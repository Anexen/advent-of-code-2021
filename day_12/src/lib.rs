// TODO: optimize

use std::collections::{HashMap, HashSet};

struct Counter(u64);

impl Counter {
    fn new() -> Self {
        Self(0)
    }
    fn inc(&mut self) {
        self.0 += 1
    }
}

// to try: &str + lifetime
struct Graph {
    adj_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, src: &str, dest: &str) {
        for (a, b) in [(src, dest), (dest, src)] {
            self.adj_list
                .entry(a.to_string())
                .or_insert(Vec::new())
                .push(b.to_string());
        }
    }
}

fn read_input(input: &str) -> Graph {
    let mut graph = Graph::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        graph.add_edge(a, b);
    });

    graph
}

fn count_a(
    graph: &Graph,
    src: &str,
    dest: &str,
    visited: &mut HashSet<String>,
    counter: &mut Counter,
) {
    if src.to_lowercase() == src {
        // we can visit big caves any number of times
        // but we should visit small caves at most once
        visited.insert(src.to_string());
    }
    if src == dest {
        counter.inc();
    } else {
        for adj in graph.adj_list.get(src).unwrap_or(&Vec::new()) {
            if !visited.contains(adj) {
                count_a(graph, adj, dest, visited, counter)
            }
        }
    }
    visited.remove(src);
}

fn count_b(
    graph: &Graph,
    src: &str,
    dest: &str,
    path: &mut Vec<String>,
    counter: &mut Counter,
    v: bool,
) {
    if src == dest {
        counter.inc();
        // println!("{:?}", path);
    } else {
        for adj in graph.adj_list.get(src).unwrap_or(&Vec::new()) {
            let a = &adj.to_lowercase() == adj && !v && path.contains(adj) && adj != "start";
            if &adj.to_uppercase() == adj || !path.contains(adj) || a {
                let v = v || a;
                path.push(adj.to_string());
                count_b(graph, adj, dest, path, counter, v);
                path.pop();
            }
        }
    }
}

fn path_count_a(graph: &Graph, src: &str, dest: &str) -> u64 {
    let mut visited = HashSet::new();
    let mut counter = Counter::new();
    count_a(&graph, src, dest, &mut visited, &mut counter);
    counter.0
}

fn path_count_b(graph: &Graph, src: &str, dest: &str) -> u64 {
    let mut counter = Counter::new();
    let mut path = vec!["start".to_string()];
    count_b(&graph, src, dest, &mut path, &mut counter, false);
    counter.0
}

pub fn part_a(input: Option<&str>) -> u64 {
    let graph = read_input(input.unwrap_or(include_str!("../input.txt")));
    path_count_a(&graph, "start", "end")
}

pub fn part_b(input: Option<&str>) -> u64 {
    let graph = read_input(input.unwrap_or(include_str!("../input.txt")));
    path_count_b(&graph, "start", "end")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_works() {
        let mut graph = super::Graph::new();
        graph.add_edge("start", "A");
        graph.add_edge("start", "b");
        graph.add_edge("A", "c");
        graph.add_edge("A", "b");
        graph.add_edge("b", "d");
        graph.add_edge("A", "end");
        graph.add_edge("b", "end");

        assert_eq!(super::path_count_a(&graph, "start", "end"), 10);
        assert_eq!(super::path_count_b(&graph, "start", "end"), 36);
    }

    #[test]
    fn test_part_a_works() {
        let result = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(result, 226);
    }

    #[test]
    fn test_part_a() {
        let result = super::part_a(None);
        assert_eq!(result, 4775);
    }

    #[test]
    fn test_part_b_works() {
        let result = super::part_b(Some(include_str!("../example.txt")));
        assert_eq!(result, 3509);
    }

    #[test]
    fn test_part_b() {
        let result = super::part_b(None);
        assert_eq!(result, 152480);
    }
}

use std::{
    collections::{HashMap, VecDeque},
    fs, iter,
};

use itertools::Itertools;

#[derive(Debug)]
struct Node {
    edges: Vec<usize>,
    is_lower: bool,
}

#[derive(Debug)]
struct Graph<'a> {
    nodes: Vec<Node>,
    name_to_idx: HashMap<&'a str, usize>,
}

impl<'a> From<&'a str> for Graph<'a> {
    fn from(s: &'a str) -> Self {
        let capacity = s.lines().count();
        let mut g = Graph {
            nodes: Vec::with_capacity(capacity),
            name_to_idx: HashMap::with_capacity(capacity),
        };
        s.lines().for_each(|l| {
            let (a, b) = l.split_once('-').unwrap();
            let i = g.nodes.len();

            let a_idx = if g.name_to_idx.contains_key(a) {
                *g.name_to_idx.get(a).unwrap()
            } else {
                i
            };

            let b_idx = if g.name_to_idx.contains_key(b) {
                *g.name_to_idx.get(b).unwrap()
            } else if a_idx == i {
                i + 1
            } else {
                i
            };

            let mut push_node = |node: &'a str, from_idx: usize, to_idx: usize| {
                if g.name_to_idx.contains_key(node) {
                    g.nodes[*g.name_to_idx.get(node).unwrap()]
                        .edges
                        .push(to_idx);
                } else {
                    g.nodes.push(Node {
                        edges: vec![to_idx],
                        is_lower: node.chars().all(|c| c.is_lowercase()),
                    });
                    g.name_to_idx.insert(node, from_idx);
                }
            };
            push_node(a, a_idx, b_idx);
            push_node(b, b_idx, a_idx);
        });
        g
    }
}

impl<'a> Graph<'a> {
    fn is_dead_p1(&self, _: usize, to_node: usize, current_path: &[usize]) -> bool {
        self.nodes[to_node].is_lower && current_path.contains(&to_node)
    }

    fn is_dead_p2(&self, start: usize, to_node: usize, current_path: &[usize]) -> bool {
        to_node == start
            || (self.nodes[to_node].is_lower
                && ((current_path.iter().filter(|f| **f == to_node).count() > 1)
                    || (current_path
                        .iter()
                        .filter(|f| self.nodes[**f].is_lower)
                        .chain(iter::once(&to_node))
                        .duplicates()
                        .count()
                        > 1)))
    }

    fn bfs(&self, is_dead: fn(&Self, usize, usize, &[usize]) -> bool) -> u32 {
        let start = *self.name_to_idx.get("start").unwrap();
        let end = *self.name_to_idx.get("end").unwrap();
        let mut to_visit: VecDeque<Vec<usize>> = VecDeque::new();
        to_visit.push_back(vec![start; 1]);
        let mut found = 0u32;
        while !to_visit.is_empty() {
            let current = to_visit.pop_front().unwrap();
            self.nodes[*current.last().unwrap()]
                .edges
                .iter()
                .for_each(|to_node| {
                    if *to_node == end {
                        found += 1;
                    } else if !is_dead(self, start, *to_node, &current) {
                        to_visit.push_back(Vec::from_iter(
                            current.iter().copied().chain(iter::once(*to_node)),
                        ));
                    }
                })
        }
        found
    }
}

fn solution(input: &str) -> u32 {
    Graph::from(input).bfs(Graph::is_dead_p1)
}

fn solution2(input: &str) -> u32 {
    Graph::from(input).bfs(Graph::is_dead_p2)
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_12_sample1.input").unwrap();
    assert_eq!(solution(&input), 10);
    assert_eq!(solution2(&input), 36);
    let input = fs::read_to_string("src/inputs/aoc_12_sample2.input").unwrap();
    assert_eq!(solution(&input), 19);
    assert_eq!(solution2(&input), 103);
    let input = fs::read_to_string("src/inputs/aoc_12_sample3.input").unwrap();
    assert_eq!(solution(&input), 226);
    assert_eq!(solution2(&input), 3509);
    let input = fs::read_to_string("src/inputs/aoc_12.input").unwrap();
    assert_eq!(solution(&input), 4659);
    assert_eq!(solution2(&input), 148962);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_12.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

use itertools::Itertools;
use std::{collections::BinaryHeap, fs};

#[derive(Debug)]
struct Grid {
    g: Vec<Vec<u8>>,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        Self {
            g: s.lines()
                .map(|l| l.chars().map(|c| c as u8 - 0x30).collect())
                .collect::<Vec<Vec<u8>>>(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
    value: usize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.value.cmp(&self.value))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl From<(usize, usize, usize)> for Point {
    fn from(p: (usize, usize, usize)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            value: p.2,
        }
    }
}

impl Grid {
    fn p2expand(&mut self) {
        // vertical
        let l = self.g.len();
        (1..5).for_each(|i| {
            (0..l).for_each(|row| {
                let new_row = self.g[row]
                    .iter()
                    .map(|x| (x + i - 1) % 9 + 1)
                    .collect_vec();
                self.g.push(new_row);
            });
        });
        // horizontal
        let l = self.g.len();
        (0..l).for_each(|row| {
            let orow = self.g[row].clone();
            (1..5).for_each(|i| {
                let row_ext = orow.iter().map(|x| (x + i - 1) % 9 + 1).collect_vec();
                self.g[row].extend(row_ext);
            });
        });
    }

    fn get_surroundings(&self, x: usize, y: usize) -> impl Iterator<Item = Point> + '_ {
        [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .iter()
            .filter_map(move |(xd, yd)| {
                let nx = x as i32 + xd;
                let ny = y as i32 + yd;
                if nx < 0
                    || ny < 0
                    || ny as usize >= self.g.len()
                    || nx as usize >= self.g[ny as usize].len()
                {
                    None
                } else {
                    Some(Point::from((
                        nx as usize,
                        ny as usize,
                        self.g[ny as usize][nx as usize] as usize,
                    )))
                }
            })
    }

    fn get_point(&self, x: usize, y: usize) -> Point {
        Point::from((x, y, self.g[y][x] as usize))
    }

    fn sol(&self) -> usize {
        let mut risks = vec![vec![usize::MAX; self.g[0].len()]; self.g.len()];
        let mut visited = vec![vec![false; self.g[0].len()]; self.g.len()];
        let mut to_visit: BinaryHeap<Point> = BinaryHeap::new();
        let start = self.get_point(0, 0);
        to_visit.push(start);
        risks[start.y][start.x] = 0;
        while !to_visit.is_empty() {
            let current = to_visit.pop().unwrap();
            if visited[current.y][current.x] {
                continue;
            }
            visited[current.y][current.x] = true;
            let cv = risks[current.y][current.x] as usize;
            self.get_surroundings(current.x, current.y).for_each(|sp| {
                if !visited[sp.y][sp.x] {
                    let spv = self.g[sp.y][sp.x] as usize;
                    if cv + spv < risks[sp.y][sp.x] {
                        risks[sp.y][sp.x] = cv + spv;
                    }
                    to_visit.push(Point::from((sp.x, sp.y, risks[sp.y][sp.x])));
                }
            });
        }
        let end = self.get_point(self.g[0].len() - 1, self.g.len() - 1);
        risks[end.y][end.x]
    }
}

fn solution(input: &str) -> usize {
    let g = Grid::from(input);
    g.sol()
}

fn solution2(input: &str) -> usize {
    let mut g = Grid::from(input);
    g.p2expand();
    println!("{} {}", g.g.len(), g.g[0].len());
    g.sol()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_15_sample.input").unwrap();
    assert_eq!(solution(&input), 40);
    assert_eq!(solution2(&input), 315);
    let input = fs::read_to_string("src/inputs/aoc_15.input").unwrap();
    assert_eq!(solution(&input), 523);
    assert_eq!(solution2(&input), 2876);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_15.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

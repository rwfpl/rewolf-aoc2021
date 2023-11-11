use std::collections::VecDeque;
use std::fs;

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

impl Grid {
    fn is_lowest(&self, x: usize, y: usize) -> bool {
        self.get_surroundings(x, y)
            .all(|(nx, ny)| self.g[ny][nx] > self.g[y][x])
    }

    fn get_lowest(&self) -> Vec<(usize, usize)> {
        (0..self.g.len())
            .flat_map(|y| {
                (0..self.g[y].len())
                    .filter(|x| self.is_lowest(*x, y))
                    .map(|x| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>()
    }

    fn get_score1(&self) -> u32 {
        self.get_lowest()
            .iter()
            .fold(0u32, |acc, (x, y)| acc + self.g[*y][*x] as u32 + 1)
    }

    fn get_surroundings(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
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
                    Some((nx as usize, ny as usize))
                }
            })
    }

    fn get_non9(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.get_surroundings(x, y)
            .filter(|(xs, ys)| self.g[*ys][*xs] != 9)
    }

    fn get_basin_size(&self, x: usize, y: usize) -> u32 {
        let mut visited = vec![vec![false; self.g[0].len()]; self.g.len()];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut basin_size = 0;
        q.push_back((x, y));
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            basin_size += 1;
            q.extend(self.get_non9(x, y).filter(|(x, y)| !visited[*y][*x]))
        }
        basin_size
    }

    fn get_score2(&self) -> u32 {
        let mut lowest = self
            .get_lowest()
            .iter()
            .map(|(x, y)| self.get_basin_size(*x, *y))
            .collect::<Vec<u32>>();
        lowest.sort_by(|a, b| b.cmp(a));
        lowest.iter().take(3).product()
    }
}

fn solution(input: &str) -> (u32, u32) {
    let g = Grid::from(input);
    (g.get_score1(), g.get_score2())
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_9_sample.input").unwrap();
    assert_eq!(solution(&input), (15, 1134));
    let input = fs::read_to_string("src/inputs/aoc_9.input").unwrap();
    assert_eq!(solution(&input), (522, 916688));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_9.input").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

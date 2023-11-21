use std::fs;

use itertools::Itertools;

#[derive(Debug)]
struct Grid {
    g: [[u8; 10]; 10],
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        Self {
            g: s.lines()
                .take(10)
                .map(|l| {
                    l.chars()
                        .take(10)
                        .map(|c| c as u8 - 0x30)
                        .collect::<Vec<u8>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[u8; 10]>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl Grid {
    fn get_surroundings(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (1, 1),
            (-1, -1),
            (1, -1),
        ]
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

    fn get_flashed(&self, flashing: &[(usize, usize)]) -> Vec<(usize, usize)> {
        flashing
            .iter()
            .flat_map(|(x, y)| self.get_surroundings(*x, *y))
            .collect::<Vec<(usize, usize)>>()
    }

    fn increase_all(&mut self) {
        self.g
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v += 1));
    }

    fn zero_flashed(&mut self) -> u64 {
        self.g
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|v| {
                        if *v == 11 {
                            *v = 0;
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    fn get10s(&self) -> Vec<(usize, usize)> {
        self.g
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(
                    move |(x, v)| {
                        if *v == 10 {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect_vec()
    }

    fn step(&mut self) -> u64 {
        self.increase_all();
        loop {
            let tens = self.get10s();
            if tens.is_empty() {
                break;
            }
            let flashed = self.get_flashed(&tens);
            // Bump all 10s to 11, so they won't be flashing again.
            tens.iter().for_each(|(x, y)| self.g[*y][*x] += 1);

            flashed.iter().for_each(|(x, y)| {
                if self.g[*y][*x] <= 9 {
                    self.g[*y][*x] += 1;
                }
            })
        }
        self.zero_flashed()
    }
}

fn solution(input: &str) -> u64 {
    let mut g = Grid::from(input);
    (0..100).map(|_| g.step()).sum()
}

fn solution2(input: &str) -> u64 {
    let mut g = Grid::from(input);
    (1..u64::MAX).find(|_| g.step() == 100).unwrap()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_11_sample.input").unwrap();
    assert_eq!(solution(&input), 1656);
    assert_eq!(solution2(&input), 195);
    let input = fs::read_to_string("src/inputs/aoc_11.input").unwrap();
    assert_eq!(solution(&input), 1649);
    assert_eq!(solution2(&input), 256);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_11.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

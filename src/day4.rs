use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Board {
    numbers: [Vec<u32>; 5],
    hits: [Vec<bool>; 5],
    bingo: bool,
}

impl Board {
    pub fn new(s: [&str; 5]) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        Self {
            numbers: s.map(|l| {
                RE.find_iter(l)
                    .map(|m| m.as_str().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            }),
            hits: [
                vec![false; 5],
                vec![false; 5],
                vec![false; 5],
                vec![false; 5],
                vec![false; 5],
            ],
            bingo: false,
        }
    }

    fn get_score(&self) -> u32 {
        self.hits
            .iter()
            .enumerate()
            .map(|(i, vec)| {
                vec.iter()
                    .enumerate()
                    .filter(|(_, val)| !**val)
                    .map(|(j, _)| self.numbers[i][j])
                    .sum::<u32>()
            })
            .sum()
    }

    fn find_number(&self, num: u32) -> Option<(usize, usize)> {
        self.numbers.iter().enumerate().find_map(|(i, val)| {
            let x = val.iter().enumerate().find(|(_, val)| **val == num);
            if let Some((j, _)) = x {
                return Some((i, j));
            }
            None
        })
    }

    fn check_bingo(&self, i: usize, j: usize) -> bool {
        (0..5).all(|j| self.hits[i][j]) || (0..5).all(|i| self.hits[i][j])
    }

    fn play(&mut self, num: u32) -> bool {
        if let Some((i, j)) = self.find_number(num) {
            self.hits[i][j] = true;
            self.bingo = self.check_bingo(i, j);
            return self.bingo;
        }
        false
    }
}

#[derive(Debug)]
struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    pub fn new(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        let mut lines = s.lines();
        Self {
            numbers: RE
                .find_iter(lines.next().unwrap())
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect(),
            boards: lines
                .chunks(6)
                .into_iter()
                .map(|mut chunk| {
                    Board::new(chunk.next_chunk::<6>().unwrap()[1..].try_into().unwrap())
                })
                .collect(),
        }
    }

    fn play(&mut self) -> u32 {
        self.numbers
            .iter()
            .map(|n| {
                self.boards
                    .iter_mut()
                    .map(|board| {
                        if board.play(*n) {
                            return board.get_score() * *n;
                        }
                        0u32
                    })
                    .find(|v| *v != 0)
            })
            .find(|v| v.is_some())
            .unwrap()
            .unwrap()
    }

    fn play2(&mut self) -> u32 {
        let x = self
            .numbers
            .iter()
            .map(|n| {
                let x = self
                    .boards
                    .iter_mut()
                    .filter(|f| !f.bingo)
                    .map(|board| {
                        if board.play(*n) {
                            return Some(board.get_score() * *n);
                        }
                        None
                    })
                    .filter(|p| p.is_some());
                if let Some(Some(y)) = x.last() {
                    return Some(y);
                }
                None
            })
            .filter(|p| p.is_some());
        x.last().unwrap().unwrap()
    }
}

fn solution(input: &str) -> u32 {
    Bingo::new(input).play()
}

fn solution2(input: &str) -> u32 {
    Bingo::new(input).play2()
}

#[test]
fn test_run() {
    let sample_input = fs::read_to_string("src/inputs/aoc_4_sample.input").unwrap();
    assert_eq!(solution(&sample_input), 4512);
    assert_eq!(solution2(&sample_input), 1924);

    let input = fs::read_to_string("src/inputs/aoc_4.input").unwrap();
    assert_eq!(solution(&input), 10374);
    assert_eq!(solution2(&input), 24742);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_4.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

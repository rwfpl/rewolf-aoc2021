use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{cmp, fs};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    begin: Point,
    end: Point,
    enable_diagonal: bool,
}

impl Line {
    fn get_points(&self) -> Option<Vec<Point>> {
        if self.begin.x == self.end.x {
            let [a, b] = cmp::minmax(self.begin.y, self.end.y);
            Some((a..=b).map(|y| Point { x: self.begin.x, y }).collect())
        } else if self.begin.y == self.end.y {
            let [a, b] = cmp::minmax(self.begin.x, self.end.x);
            Some((a..=b).map(|x| Point { x, y: self.begin.y }).collect())
        } else if self.enable_diagonal {
            let length = (self.begin.y as i32 - self.end.y as i32).abs();
            let y_dir = (self.end.y as i32 - self.begin.y as i32).signum();
            let x_dir = (self.end.x as i32 - self.begin.x as i32).signum();
            Some(
                (0..=length)
                    .map(|i| Point {
                        x: (self.begin.x as i32 + i * x_dir) as usize,
                        y: (self.begin.y as i32 + i * y_dir) as usize,
                    })
                    .collect::<Vec<Point>>(),
            )
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<u32>>,
}

fn draw_line(tiles: &mut [Vec<u32>], line: &Line) {
    if let Some(p) = line.get_points() {
        p.iter().for_each(|p| tiles[p.y][p.x] += 1)
    }
}

impl Board {
    fn new(lines: &[Line]) -> Self {
        let max_x = lines
            .iter()
            .map(|l| cmp::max(l.begin.x, l.end.x))
            .max()
            .unwrap();
        let max_y = lines
            .iter()
            .map(|l| cmp::max(l.begin.y, l.end.y))
            .max()
            .unwrap();
        let mut tiles = vec![vec![0; max_x + 1]; max_y + 1];
        lines.iter().for_each(|line| draw_line(&mut tiles, line));
        Board { tiles }
    }

    fn get_score(&self) -> u32 {
        self.tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| if *v > 1 { 1u32 } else { 0u32 })
                    .sum::<u32>()
            })
            .sum()
    }
}

fn get_usize(caps: &Captures, n: usize) -> usize {
    caps.get(n)
        .map(|v| str::parse(v.as_str()).unwrap())
        .unwrap()
}

fn solution(input: &str, enable_diagonal: bool) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    }
    let board = Board::new(
        &input
            .lines()
            .map(|l| {
                RE.captures(l)
                    .map(|caps| Line {
                        begin: Point {
                            x: get_usize(&caps, 1),
                            y: get_usize(&caps, 2),
                        },
                        end: Point {
                            x: get_usize(&caps, 3),
                            y: get_usize(&caps, 4),
                        },
                        enable_diagonal,
                    })
                    .unwrap()
            })
            .collect::<Vec<Line>>(),
    );
    board.get_score()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_5_sample.input").unwrap();
    assert_eq!(solution(&input, false), 5);
    assert_eq!(solution(&input, true), 12);
    let input = fs::read_to_string("src/inputs/aoc_5.input").unwrap();
    assert_eq!(solution(&input, false), 5698);
    assert_eq!(solution(&input, true), 15463);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_5.input").unwrap();
    (
        solution(&input, false).to_string(),
        solution(&input, true).to_string(),
    )
}

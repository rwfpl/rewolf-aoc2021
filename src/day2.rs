use std::fs;
use std::ops::Add;

#[derive(Debug)]
enum Move {
    Forward(i32),
    Up(i32),
    Down(i32),
    Invalid,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s.split_once(' ') {
            Some(("forward", v)) => Move::Forward(v.parse::<i32>().unwrap()),
            Some(("down", v)) => Move::Down(v.parse::<i32>().unwrap()),
            Some(("up", v)) => Move::Up(v.parse::<i32>().unwrap()),
            _ => Move::Invalid,
        }
    }
}

#[derive(Debug)]
struct Position {
    h: i32,
    d: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            h: self.h + other.h,
            d: self.d + other.d,
        }
    }
}

fn solution(input: &str) -> i32 {
    let x: Position = input
        .lines()
        .map(Move::from)
        .map(|m| match m {
            Move::Forward(v) => Position { h: v, d: 0 },
            Move::Down(v) => Position { h: 0, d: v },
            Move::Up(v) => Position { h: 0, d: -v },
            Move::Invalid => Position { h: 0, d: 0 },
        })
        .fold(Position { h: 0, d: 0 }, |a, b| a + b);
    x.d * x.h
}

#[derive(Debug)]
struct Position2 {
    h: i32,
    d: i32,
    aim: i32,
}

impl Position2 {
    fn up(&self, v: i32) -> Self {
        Self {
            h: self.h,
            d: self.d,
            aim: self.aim - v,
        }
    }
    fn down(&self, v: i32) -> Self {
        Self {
            h: self.h,
            d: self.d,
            aim: self.aim + v,
        }
    }
    fn forward(&self, v: i32) -> Self {
        Self {
            h: self.h + v,
            d: self.d + v * self.aim,
            aim: self.aim,
        }
    }
}

fn solution2(input: &str) -> i32 {
    let x: Position2 =
        input
            .lines()
            .map(Move::from)
            .fold(Position2 { h: 0, d: 0, aim: 0 }, |a, b| match b {
                Move::Forward(v) => a.forward(v),
                Move::Down(v) => a.down(v),
                Move::Up(v) => a.up(v),
                Move::Invalid => a,
            });
    x.d * x.h
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_2_sample.input").unwrap();
    assert_eq!(solution(&input), 150);
    assert_eq!(solution2(&input), 900);
    let input = fs::read_to_string("src/inputs/aoc_2.input").unwrap();
    assert_eq!(solution(&input), 1813801);
    assert_eq!(solution2(&input), 1960569556);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_2.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

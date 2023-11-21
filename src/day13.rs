use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (sx, sy) = s.split_once(',').unwrap();
        Self {
            x: str::parse(sx).unwrap(),
            y: str::parse(sy).unwrap(),
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl From<&str> for Fold {
    fn from(s: &str) -> Self {
        let (axis, v) = s
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();
        match axis {
            "x" => Fold::X(str::parse(v).unwrap()),
            "y" => Fold::Y(str::parse(v).unwrap()),
            _ => panic!("!nicht gut!"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Point(Point),
    Fold(Fold),
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s.chars().next().unwrap() {
            'f' => Action::Fold(Fold::from(s)),
            _ => Action::Point(Point::from(s)),
        }
    }
}

fn points_stringify(points: &HashSet<Point>) -> String {
    let max_x = points.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p.y).max().unwrap() + 1;
    (0..max_y)
        .flat_map(|y| {
            std::iter::once('\n').chain((0..max_x).map(move |x| {
                if points.contains(&Point { x, y }) {
                    'X'
                } else {
                    ' '
                }
            }))
        })
        .collect::<String>()
}

fn solution(input: &str) -> (usize, String) {
    let (points, folds): (Vec<Action>, Vec<Action>) = input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(Action::from(l))
            }
        })
        .partition(|a| matches!(a, Action::Point(_)));

    let mut after_first_fold_count = 0;
    let mut points = points
        .iter()
        .filter_map(|p| {
            if let Action::Point(pp) = p {
                Some(*pp)
            } else {
                None
            }
        })
        .collect::<HashSet<Point>>();
    for f in folds {
        if let Action::Fold(f) = f {
            points = points
                .iter()
                .map(|p| match f {
                    Fold::X(v) => {
                        if p.x <= v {
                            *p
                        } else {
                            Point {
                                y: p.y,
                                x: p.x - (p.x - v) * 2,
                            }
                        }
                    }
                    Fold::Y(v) => {
                        if p.y <= v {
                            *p
                        } else {
                            Point {
                                x: p.x,
                                y: p.y - (p.y - v) * 2,
                            }
                        }
                    }
                })
                .collect::<HashSet<Point>>();
            if after_first_fold_count == 0 {
                after_first_fold_count = points.len();
            }
        }
    }
    (after_first_fold_count, points_stringify(&points))
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_13_sample.input").unwrap();
    assert_eq!(
        solution(&input),
        (
            17,
            r"
XXXXX
X   X
X   X
X   X
XXXXX"
                .to_string()
        )
    );
    let input = fs::read_to_string("src/inputs/aoc_13.input").unwrap();
    assert_eq!(
        solution(&input),
        (
            785,
            r"
XXXX   XX  XX  X  X   XX  XX   XX  X  X
X       X X  X X  X    X X  X X  X X  X
XXX     X X  X XXXX    X X    X  X XXXX
X       X XXXX X  X    X X XX XXXX X  X
X    X  X X  X X  X X  X X  X X  X X  X
X     XX  X  X X  X  XX   XXX X  X X  X"
                .to_string()
        )
    );
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_13.input").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2)
}

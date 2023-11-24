use itertools::FoldWhile;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::iter;

#[derive(Debug)]
struct Bucket {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Bucket {
    fn area(&self) -> usize {
        ((self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1)) as usize
    }

    fn contains(&self, p: (i32, i32)) -> bool {
        p.0 >= self.x_min && p.0 <= self.x_max && p.1 >= self.y_min && p.1 <= self.y_max
    }

    fn simulate(&self, v: (i32, i32)) -> bool {
        iter::repeat(0)
            .fold_while(((0, 0), v), |(cp, cv), _| {
                if self.contains(cp) {
                    FoldWhile::Done(((1, 1), (1, 1)))
                } else if cp.0 > self.x_max || cp.1 < self.y_min {
                    FoldWhile::Done(((0, 0), (0, 0)))
                } else {
                    FoldWhile::Continue((
                        (cp.0 + cv.0, cp.1 + cv.1),
                        (std::cmp::max(cv.0 - 1, 0), cv.1 - 1),
                    ))
                }
            })
            .into_inner()
            .0
            == (1, 1)
    }
}

impl From<&str> for Bucket {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Self {
            x_min: (caps[1]).parse::<i32>().unwrap(),
            x_max: (caps[2]).parse::<i32>().unwrap(),
            y_min: (caps[3]).parse::<i32>().unwrap(),
            y_max: (caps[4]).parse::<i32>().unwrap(),
        }
    }
}

fn solution(input: &str) -> (i32, usize) {
    let b = Bucket::from(input);
    let min_x_velocity = ((-1.0 + ((1 + 4 * 2 * b.x_min) as f32).sqrt()) / 2.0).ceil() as i32;
    let p2 = b.area()
        + (min_x_velocity..b.x_min)
            .map(|x| (b.y_max..100).rev().filter(|y| b.simulate((x, *y))).count())
            .sum::<usize>();
    let p1_max = std::cmp::max(b.y_min.abs(), b.y_max.abs());
    ((p1_max - 1) * p1_max / 2, p2)
}

#[test]
fn test_run() {
    assert_eq!(solution("target area: x=20..30, y=-10..-5"), (45, 112));
    assert_eq!(
        solution("target area: x=137..171, y=-98..-73"),
        (4753, 1546)
    );
}

pub fn run() -> (String, String) {
    let input = "target area: x=137..171, y=-98..-73";
    let (p1, p2) = solution(input);
    (p1.to_string(), p2.to_string())
}

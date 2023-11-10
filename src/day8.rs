use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn solution(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-g]+").unwrap();
    }
    input
        .lines()
        .map(|l| {
            RE.find_iter(l)
                .skip(10)
                .filter(|d| [2, 3, 4, 7].contains(&d.as_str().len()))
                .count()
        })
        .sum()
}

struct Digit {
    s: String,
    hs: HashSet<char>,
}

struct ParsedInput {
    digits: Vec<Digit>,
    output_values: Vec<Digit>,
}

impl ParsedInput {
    fn from_tuple(t: (Vec<Digit>, Vec<Digit>)) -> Self {
        Self {
            digits: t.0,
            output_values: t.1,
        }
    }
}
type DigitMap<'a> = HashMap<&'a String, (u32, &'a HashSet<char>)>;

fn parse_line(l: &str) -> ParsedInput {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-g]+").unwrap();
    }
    ParsedInput::from_tuple(
        l.split('|')
            .take(2)
            .map(|s| {
                RE.find_iter(s)
                    .map(|t| Digit {
                        s: t.as_str().chars().sorted().collect::<String>(),
                        hs: HashSet::<char>::from_iter(t.as_str().chars()),
                    })
                    .collect::<Vec<Digit>>()
            })
            .collect_tuple()
            .unwrap(),
    )
}

fn map1478(pi: &ParsedInput) -> DigitMap {
    pi.digits
        .iter()
        .filter_map(|d| match d.s.len() {
            2 => Some((&d.s, (1, &d.hs))),
            4 => Some((&d.s, (4, &d.hs))),
            3 => Some((&d.s, (7, &d.hs))),
            7 => Some((&d.s, (8, &d.hs))),
            _ => None,
        })
        .collect::<DigitMap>()
}

fn map235<'a>(pi: &'a ParsedInput, digit_map: &DigitMap) -> DigitMap<'a> {
    let rmap = digit_map
        .iter()
        .map(|(_, (d, hs))| (d, hs))
        .collect::<HashMap<&u32, &&HashSet<char>>>();
    // find 2, 3, 5
    pi.digits
        .iter()
        .filter(|d| d.s.len() == 5)
        .map(|d| {
            if d.hs.is_superset(rmap.get(&1).unwrap()) {
                (&d.s, (3, &d.hs))
            } else if d.hs.is_superset(
                &rmap
                    .get(&4)
                    .unwrap()
                    .difference(rmap.get(&1).unwrap())
                    .cloned()
                    .collect::<HashSet<char>>(),
            ) {
                (&d.s, (5, &d.hs))
            } else {
                (&d.s, (2, &d.hs))
            }
        })
        .collect::<DigitMap>()
}

fn map069<'a>(pi: &'a ParsedInput, digit_map: &DigitMap) -> DigitMap<'a> {
    let rmap = digit_map
        .iter()
        .map(|(_, (d, hs))| (d, hs))
        .collect::<HashMap<&u32, &&HashSet<char>>>();
    pi.digits
        .iter()
        .filter(|d| d.s.len() == 6)
        .map(|d| {
            if d.hs.eq(&rmap
                .get(&3)
                .unwrap()
                .union(rmap.get(&4).unwrap())
                .cloned()
                .collect::<HashSet<char>>())
            {
                (&d.s, (9, &d.hs))
            } else if d.hs.is_superset(rmap.get(&5).unwrap()) {
                (&d.s, (6, &d.hs))
            } else {
                (&d.s, (0, &d.hs))
            }
        })
        .collect::<DigitMap>()
}

fn process_line(l: &str) -> u32 {
    let pi = parse_line(l);

    let mut digit_map = map1478(&pi);
    digit_map.extend(map235(&pi, &digit_map));
    digit_map.extend(map069(&pi, &digit_map));
    pi.output_values
        .iter()
        .map(|d| digit_map.get(&d.s).unwrap().0)
        .fold(0, |acc, v| acc * 10 + v)
}

fn solution2(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_8_sample.input").unwrap();
    assert_eq!(solution(&input), 26);
    assert_eq!(solution2(&input), 61229);
    let input = fs::read_to_string("src/inputs/aoc_8.input").unwrap();
    assert_eq!(solution(&input), 330);
    assert_eq!(solution2(&input), 1010472);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_8.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

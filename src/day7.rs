use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn solution(input: &str, fuel: fn(u32) -> usize) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let crabs = RE
        .find_iter(input)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let max_crab = crabs.iter().max().unwrap();
    (0..=*max_crab)
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| fuel(crab.abs_diff(pos)))
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

fn solution1(input: &str) -> usize {
    solution(input, |x| x as usize)
}

fn solution2(input: &str) -> usize {
    solution(input, |x| (((1 + x) * x) / 2) as usize)
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_7_sample.input").unwrap();
    assert_eq!(solution1(&input), 37);
    assert_eq!(solution2(&input), 168);
    let input = fs::read_to_string("src/inputs/aoc_7.input").unwrap();
    assert_eq!(solution1(&input), 339321);
    assert_eq!(solution2(&input), 95476244);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_7.input").unwrap();
    (solution1(&input).to_string(), solution2(&input).to_string())
}

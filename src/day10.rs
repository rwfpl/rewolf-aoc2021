use std::collections::VecDeque;
use std::fs;

fn score_char(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_missing_char(c: char) -> u32 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn is_match(c: char, m: char) -> bool {
    match c {
        ')' => m == '(',
        ']' => m == '[',
        '}' => m == '{',
        '>' => m == '<',
        _ => false,
    }
}

fn is_open(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn score_line(l: &str) -> (u32, u64) {
    let mut q: VecDeque<char> = VecDeque::new();
    let score_p1 = l
        .chars()
        .filter_map(|c| {
            if is_open(c) {
                q.push_back(c);
                None
            } else {
                let pb = q.pop_back().unwrap_or('X');
                if is_match(c, pb) {
                    None
                } else {
                    Some(score_char(c))
                }
            }
        })
        .sum();
    if score_p1 != 0 {
        (score_p1, 0)
    } else {
        (
            0,
            q.iter()
                .rev()
                .map(|c| score_missing_char(*c))
                .fold(0u64, |acc, sc| acc * 5 + sc as u64),
        )
    }
}

fn solution(input: &str) -> (u32, u64) {
    let scores: Vec<(u32, u64)> = input.lines().map(score_line).collect();
    let mut scores_p2: Vec<u64> = scores
        .iter()
        .filter(|(_, p2)| *p2 != 0)
        .map(|(_, p2)| *p2)
        .collect();
    scores_p2.sort();
    (
        scores.iter().map(|s| s.0).sum(),
        scores_p2[scores_p2.len() / 2],
    )
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_10_sample.input").unwrap();
    assert_eq!(solution(&input), (26397, 288957));
    let input = fs::read_to_string("src/inputs/aoc_10.input").unwrap();
    assert_eq!(solution(&input), (266301, 3404870164));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_10.input").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}

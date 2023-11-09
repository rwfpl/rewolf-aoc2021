use std::fs;

fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map_windows(|[a, b]| b > a)
        .filter(|x| *x)
        .count()
}

fn solution2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map_windows(|[a, b, c]| a + b + c)
        .map_windows(|[a, b]| b > a)
        .filter(|x| *x)
        .count()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_1_sample.input").unwrap();
    assert_eq!(solution(&input), 7);
    assert_eq!(solution2(&input), 5);
    let input = fs::read_to_string("src/inputs/aoc_1.input").unwrap();
    assert_eq!(solution(&input), 1559);
    assert_eq!(solution2(&input), 1600);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_1.input").unwrap();
    (solution(&input).to_string(), solution2(&input).to_string())
}

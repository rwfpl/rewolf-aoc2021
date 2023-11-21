use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Fish {
    days_to_repro: i32,
    n: i32,
}

impl Fish {
    fn dec(&mut self) -> bool {
        if self.days_to_repro > 0 {
            self.days_to_repro -= 1;
            false
        } else {
            self.days_to_repro = 6;
            true
        }
    }
}

#[derive(Debug)]
struct Population {
    p: Vec<Fish>,
}

impl Population {
    fn daypass(&mut self) {
        let new_fish = self
            .p
            .iter_mut()
            .filter_map(|f| if f.dec() { Some(f.n) } else { None })
            .sum::<i32>();
        self.p.push(Fish {
            days_to_repro: 8,
            n: new_fish,
        });
    }

    fn get_size(&self) -> i64 {
        self.p.iter().map(|f| f.n as i64).sum::<i64>()
    }
}

fn simulate_population(max_days: i32, days_to_pass: i32) -> HashMap<i32, i64> {
    let mut p = Population {
        p: vec![
            Fish {
                days_to_repro: 1,
                n: 1,
            };
            1
        ],
    };
    (0..days_to_pass - max_days).for_each(|_| {
        p.daypass();
    });
    (0..max_days)
        .map(|i| {
            p.daypass();
            (max_days - i, p.get_size())
        })
        .collect()
}

fn solution(input: &str, days: i32) -> i64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let cache: HashMap<i32, i64> = simulate_population(5, days);
    RE.find_iter(input)
        .map(|m| cache.get(&m.as_str().parse::<i32>().unwrap()).unwrap())
        .sum()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_6_sample.input").unwrap();
    assert_eq!(solution(&input, 18), 26);
    assert_eq!(solution(&input, 80), 5934);
    assert_eq!(solution(&input, 256), 26984457539);
    let input = fs::read_to_string("src/inputs/aoc_6.input").unwrap();
    assert_eq!(solution(&input, 80), 353079);
    assert_eq!(solution(&input, 256), 1605400130036);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_6.input").unwrap();
    (
        solution(&input, 80).to_string(),
        solution(&input, 256).to_string(),
    )
}

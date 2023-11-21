use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

type Pair = [char; 2];

#[derive(Debug)]
struct PairRule {
    p: Pair,
    i: char,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PairLevel {
    p: Pair,
    level: u32,
}

type CharCountMap = HashMap<char, u64>;
type RuleMap = HashMap<Pair, char>;

lazy_static::lazy_static! {
    static ref VALUECACHE: Mutex<HashMap<PairLevel, CharCountMap>> = Mutex::new(HashMap::new());
}

impl From<&str> for PairRule {
    fn from(s: &str) -> Self {
        let a = s.chars().nth(0).unwrap();
        let b = s.chars().nth(1).unwrap();
        let c = s.chars().nth(6).unwrap();
        Self { p: [a, b], i: c }
    }
}

fn merge_maps(m1: &mut CharCountMap, m2: &CharCountMap) {
    m2.iter().for_each(|(k, v)| *m1.entry(*k).or_default() += v);
}

fn simulate(level: u32, max_level: u32, p: &Pair, rules: &RuleMap) -> CharCountMap {
    if let Some(r) = VALUECACHE.lock().unwrap().get(&PairLevel { p: *p, level }) {
        return r.clone();
    }
    let rule = rules.get(p).unwrap();
    let mut m1 = HashMap::from([(*rule, 1)]);
    if level != max_level {
        merge_maps(
            &mut m1,
            &simulate(level + 1, max_level, &[p[0], *rule], rules),
        );
        merge_maps(
            &mut m1,
            &simulate(level + 1, max_level, &[*rule, p[1]], rules),
        );
    }
    VALUECACHE
        .lock()
        .unwrap()
        .insert(PairLevel { p: *p, level }, m1.clone());
    m1
}

fn solution(input: &str, max_steps: u32) -> u64 {
    VALUECACHE.lock().unwrap().clear();
    let initial_polymer = input.lines().next().unwrap().chars().collect::<String>();
    let rules = input
        .lines()
        .skip(2)
        .map(PairRule::from)
        .map(|pr| (pr.p, pr.i))
        .collect::<RuleMap>();

    let mut cm = CharCountMap::new();
    initial_polymer
        .chars()
        .for_each(|c| *cm.entry(c).or_default() += 1);

    initial_polymer
        .chars()
        .map_windows(|[a, b]| [*a, *b])
        .for_each(|p| {
            merge_maps(&mut cm, &simulate(1, max_steps, &p, &rules));
        });
    cm.values().max().unwrap() - cm.values().min().unwrap()
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_14_sample.input").unwrap();
    assert_eq!(solution(&input, 10), 1588);
    assert_eq!(solution(&input, 40), 2188189693529);
    let input = fs::read_to_string("src/inputs/aoc_14.input").unwrap();
    assert_eq!(solution(&input, 10), 3555);
    assert_eq!(solution(&input, 40), 4439442043739);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_14.input").unwrap();
    (
        solution(&input, 10).to_string(),
        solution(&input, 40).to_string(),
    )
}

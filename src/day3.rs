use std::fs;

fn update<const T: usize>(a: &[u32; T], v: &str) -> [u32; T] {
    (0..v.len())
        .map(|x| a[x] + ((v.as_bytes()[x] as u32) & 1))
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}

fn solution<const T: usize>(input: &str) -> u32 {
    let (gamma, count) = input.lines().fold(([0u32; T], 0u32), |acc, x| {
        (update::<T>(&acc.0, x), acc.1 + 1)
    });
    let x: u32 = gamma
        .iter()
        .map(|x| (*x > count / 2) as u32)
        .fold(0u32, |acc, x| (acc << 1) | x);
    x * ((!x) & ((1u32 << T) - 1))
}

fn run_for_column<'a>(input: impl Iterator<Item = &'a str>, n: usize) -> (u32, u32) {
    input
        .map(|x| (x.as_bytes()[n] as u32) & 1)
        .fold((0u32, 0u32), |acc, x| (acc.0 + x, acc.1 + 1))
}

enum Rating {
    Oxygen,
    Co2,
}

fn col2rec<'a, const T: usize>(
    cols: impl Iterator<Item = &'a str> + Clone,
    rating: Rating,
    c: usize,
) -> Vec<&'a str> {
    if c == T {
        return cols.collect::<Vec<&str>>();
    }
    let (bits_set, count) = run_for_column(cols.clone(), c);
    if count == 1 {
        return cols.collect::<Vec<&str>>();
    }
    let (most_common, least_common) = if bits_set >= (count - bits_set) {
        ('1', '0')
    } else {
        ('0', '1')
    };
    col2rec::<T>(
        cols.filter(|s| {
            s.as_bytes()[c]
                == (match rating {
                    Rating::Oxygen => most_common,
                    Rating::Co2 => least_common,
                }) as u8
        })
        .collect::<Vec<&str>>()
        .into_iter(),
        rating,
        c + 1,
    )
}

fn solution2<const T: usize>(input: &str) -> u32 {
    let oxygen = u32::from_str_radix(col2rec::<T>(input.lines(), Rating::Oxygen, 0)[0], 2).unwrap();
    let co2 = u32::from_str_radix(col2rec::<T>(input.lines(), Rating::Co2, 0)[0], 2).unwrap();
    oxygen * co2
}

#[test]
fn test_run() {
    let input = fs::read_to_string("src/inputs/aoc_3_sample.input").unwrap();
    assert_eq!(solution::<5>(&input), 198);
    assert_eq!(solution2::<5>(&input), 230);
    let input = fs::read_to_string("src/inputs/aoc_3.input").unwrap();
    assert_eq!(solution::<12>(&input), 2498354);
    assert_eq!(solution2::<12>(&input), 3277956);
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_3.input").unwrap();
    (
        solution::<12>(&input).to_string(),
        solution2::<12>(&input).to_string(),
    )
}

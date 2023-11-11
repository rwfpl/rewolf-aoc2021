#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(cmp_minmax)]

use rayon::prelude::*;
use std::{env, time::Instant};

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let days = [
        day5::run,
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
        day10::run,
    ];
    let now = Instant::now();
    let day = env::args()
        .nth(1)
        .unwrap_or("0".to_string())
        .parse::<usize>()
        .unwrap_or(0);
    match day {
        1..=25 => {
            let (p1, p2) = days[day - 1]();
            println!("day{day} p1: {p1}\nday{day} p2: {p2}");
        }
        _ => days.par_iter().enumerate().for_each(|(i, day)| {
            let now = Instant::now();
            let (p1, p2) = day();
            let day_n = i + 1;
            println!(
                "day{day_n} p1: {p1}\nday{day_n} p2: {p2}\nday{day_n} execution time: {:?}",
                now.elapsed()
            );
        }),
    }
    println!("total execution time: {:?}", now.elapsed());
}

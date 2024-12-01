use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split_whitespace();
            (
                s.next().unwrap().parse::<u32>().unwrap(),
                s.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[(u32, u32)]) -> u32 {
    let (mut a, mut b): (Vec<u32>, Vec<u32>) = input.iter().copied().unzip();
    a.sort();
    b.sort();
    std::iter::zip(a, b)
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    let mut m = HashMap::<_, u32>::new();
    for (_, i) in input {
        *m.entry(*i).or_default() += 1;
    }

    let mut r = 0;

    for (i, _) in input {
        r += *i * m.get(i).copied().unwrap_or_default()
    }

    r
}

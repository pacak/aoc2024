use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Stones {
    One(usize),
    Two(usize, usize),
}

fn rules(n: usize) -> Stones {
    if n == 0 {
        return Stones::One(1);
    }
    let log10 = n.ilog10();
    if n >= 10 && log10 % 2 == 1 {
        let mask = 10usize.pow(log10 / 2 + 1);
        let a = n / mask;
        let b = n % mask;
        Stones::Two(a, b)
    } else {
        Stones::One(n.checked_mul(2024).unwrap())
    }
}

impl IntoIterator for Stones {
    type Item = usize;

    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Stones::One(a) => vec![a].into_iter(),
            Stones::Two(a, b) => vec![a, b].into_iter(),
        }
    }
}

#[aoc(day11, part1, brute)]
fn part1(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    let mut output = Vec::new();
    for _ in 0..25 {
        output.extend(input.drain(..).flat_map(rules));
        std::mem::swap(&mut input, &mut output);
    }
    input.len()
}

#[aoc(day11, part1, smort)]
fn part1s(input: &[usize]) -> usize {
    let mut stones = HashMap::<usize, usize>::new();
    for s in input {
        *stones.entry(*s).or_default() += 1;
    }

    let mut output = HashMap::new();
    for _ in 0..25 {
        for (k, n) in stones.drain() {
            match rules(k) {
                Stones::One(k) => {
                    *output.entry(k).or_default() += n;
                }
                Stones::Two(k1, k2) => {
                    *output.entry(k1).or_default() += n;
                    *output.entry(k2).or_default() += n;
                }
            }
        }
        std::mem::swap(&mut stones, &mut output);
    }

    stones.values().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_work() {
        assert_eq!(rules(10), Stones::Two(1, 0));
    }

    #[test]
    fn part1_example() {
        part1(&parse("0 1 10 99 999"));
        assert_eq!(part1(&parse("125 17")), 55312);
        assert_eq!(part1s(&parse("125 17")), 55312);
    }
}

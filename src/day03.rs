use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3, part1)]
fn parse1(input: &str) -> Vec<(u32, u32)> {
    let mut res = Vec::new();
    let mul = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    for c in mul.captures_iter(input) {
        res.push((c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap()));
    }
    res
}

#[aoc(day3, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| a * b).sum()
}

#[test]
fn parse1_works() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    todo!("{:?}", part1(&parse1(input)));
}

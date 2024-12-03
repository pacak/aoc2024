use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3, part1)]
fn parse1(input: &str) -> Vec<(u32, u32)> {
    let mut res = Vec::new();
    // this gets compiled several times, but it is small so don't care :)
    let mul = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    for c in mul.captures_iter(input) {
        res.push((dec(&c[1]), dec(&c[2])));
    }
    res
}

fn dec(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

#[aoc_generator(day3, part2)]
fn parse2(input: &str) -> Vec<(u32, u32)> {
    let mut res = Vec::new();
    let mut enabled = true;
    // this gets compiled several times, but it is small so don't care :)
    let args = "(?:(\\d{1,3}),(\\d{1,3}))?";
    let r = format!("(do|don't|mul)\\({args}\\)");

    let mul = Regex::new(&r).unwrap();
    for c in mul.captures_iter(input) {
        if &c[1] == "do" {
            enabled = true;
        } else if &c[1] == "don't" {
            enabled = false;
        } else if &c[1] == "mul" && enabled {
            res.push((dec(&c[2]), dec(&c[3])));
        }
    }
    res
}

#[aoc(day3, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| a * b).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|(a, b)| a * b).sum()
}

#[test]
fn parse1_works() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(161, part1(&parse1(input)));
}

#[test]
fn parse2_works() {
    // grrrrrrrrr. Different example for part 2, shame on whoever. Wasted a bunch of time.
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, part2(&parse2(input)));
}

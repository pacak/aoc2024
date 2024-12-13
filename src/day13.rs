use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

use crate::Point;

#[derive(Debug, Copy, Clone)]
struct Problem {
    a: Point,
    b: Point,
    prize: Point,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Problem> {
    let mut res = Vec::new();
    let reg = Regex::new(
        "\
Button A: X\\+(\\d+), Y\\+(\\d+)
Button B: X\\+(\\d+), Y\\+(\\d+)
Prize: X=(\\d+), Y=(\\d+)",
    )
    .unwrap();
    let num = |s: &str| s.parse::<usize>().unwrap();
    for c in reg.captures_iter(input) {
        let a = Point::new(num(&c[1]), num(&c[2]));
        let b = Point::new(num(&c[3]), num(&c[4]));
        let prize = Point::new(num(&c[5]), num(&c[6]));
        res.push(Problem { a, b, prize });
    }

    res
}

/// Brute force system of linear equations
///
/// You estimate that each button would need to be pressed no more than 100 times to win a prize.
/// How else would someone be expected to play?
///
/// You also estimate that this is not going to work for part 2.
fn brute(problem: Problem) -> Option<usize> {
    let mut best = None;
    for a in 0..100usize {
        for b in 0..100usize {
            if problem.a * a + problem.b * b == problem.prize {
                let this = Some(a * 3 + b);
                if best.is_none() {
                    best = this;
                } else {
                    best = best.min(this);
                }
            }
        }
    }
    best
}

#[aoc(day13, part1)]
fn part1(input: &[Problem]) -> usize {
    input.iter().filter_map(|p| brute(*p)).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Problem]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(part1(&parse(input)), 480);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}

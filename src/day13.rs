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

fn semismort(problem: Problem, adj: usize) -> Option<usize> {
    let ax = problem.a.x as usize;
    let ay = problem.a.y as usize;

    let bx = problem.b.x as usize;
    let by = problem.b.y as usize;

    let px = problem.prize.x as usize + adj;
    let py = problem.prize.y as usize + adj;

    // ax * a + bx * b == px     =>  ax  bx | px
    // ay * a + by * b == py         ay  by | py

    let (c11, c12, v1) = (ax as f64, bx as f64, px as f64);
    let (c21, c22, v2) = (ay as f64, by as f64, py as f64);

    // change the first line to 1 bx/ax | px/ax
    let (c11, c12, v1) = (1.0, c12 / c11, v1 / c11);

    //   1 c12 | v1
    // c21 c22 | v2

    // subtract  (1)*c21 from (2) to get 0 in the first item

    let (c21, c22, v2) = (0.0, c22 - c12 * c21, v2 - v1 * c21);

    //   1  c12 | v1
    //   0  c22 | v2

    // divide 2nd by c22 to get 0 1 |v2
    assert_ne!(c22, 0.0);

    let (c21, _c22, v2) = (c21 / c22, 1.0, v2 / c22);

    // 1 c12 | v1
    // 0   1 | v2

    // subtract (2) * c12 from (1) to get 1 0 | v1
    let (_c12, _c22, v1) = (c11 - c21 * c12, 0, v1 - c12 * v2);

    // 1 0 | v1
    // 0 1 | v2
    // v1, v2 is the answer in f64 form

    // we got an answer, but in f64 form. Real answer is probably around here.
    for dx in [-1.0, 0.0, 1.0] {
        for dy in [-1.0, 0.0, 1.0] {
            let (a, b) = ((v1 + dx) as usize, (v2 + dy) as usize);

            if ax * a + bx * b == px && ay * a + by * b == py {
                return Some(a * 3 + b);
            }
        }
    }

    None
}

#[aoc(day13, part1, brute)]
fn part1b(input: &[Problem]) -> usize {
    input.iter().filter_map(|p| brute(*p)).sum()
}

#[aoc(day13, part1, semismort)]
fn part1s(input: &[Problem]) -> usize {
    input.iter().filter_map(|p| semismort(*p, 0)).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Problem]) -> usize {
    input
        .iter()
        .filter_map(|p| semismort(*p, 10_000_000_000_000))
        .sum()
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

        assert_eq!(part1s(&parse(input)), 480);
    }

    #[test]
    fn part2_example() {
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

        assert_eq!(part2(&parse(input)), 875318608908);
    }
}

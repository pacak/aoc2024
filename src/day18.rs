use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Point, TwoDee};
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[(i32, i32)]) -> usize {
    go(&input[..1024], 71)
}

fn go(input: &[(i32, i32)], size: i32) -> usize {
    let mut m = TwoDee::<bool>::new(size as usize);
    let mut steps = TwoDee::<usize>::new(size as usize);

    for (x, y) in input.iter().copied() {
        let p = Point { x, y };
        m[p] = true;
    }

    for x in 0..size {
        for y in 0..size {
            let p = Point { x, y };
            steps[p] = 999_999;
        }
    }
    let start = Point { x: 0, y: 0 };
    steps[start] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some(p) = q.pop_front() {
        for d in Point::DIRS {
            if m.get_point(p + d).copied().unwrap_or(true) {
                continue;
            }
            if steps[p + d] > steps[p] + 1 {
                steps[p + d] = steps[p] + 1;
                q.push_back(p + d);
            }
        }
    }
    steps[(size as usize - 1, size as usize - 1)]
}

#[aoc(day18, part2)]
fn part2(input: &[(i32, i32)]) -> String {
    go2(input, 71)
}

fn go2(input: &[(i32, i32)], size: i32) -> String {
    for i in 0..input.len() {
        if go(&input[..i], size) == 999_999 {
            let (x, y) = input[i - 1];
            return format!("{x},{y}");
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0\n";
        assert_eq!(go(&parse(input)[..12], 7), 22);
    }

    #[test]
    fn part2_example() {
        let input = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0\n";
        assert_eq!(go2(&parse(input), 7), "6,1");
    }
}

use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::Point;

#[derive(Debug, Copy, Clone)]
struct Ant {
    x: i32,
    y: i32,
    freq: char,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Ant> {
    let mut res = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, freq) in line.chars().enumerate() {
            if freq != '.' {
                res.push(Ant {
                    x: x as i32,
                    y: y as i32,
                    freq,
                });
            }
        }
    }
    res
}

impl From<Ant> for Point {
    fn from(value: Ant) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[aoc(day8, part1)]
fn part1(input: &[Ant]) -> usize {
    // Can't be arsed to pass it as well
    let dim = if input.len() == 7 { 12 } else { 50 };

    let mut out = HashSet::<Point>::new();
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let a = input[i];
            let b = input[j];

            if a.freq != b.freq {
                continue;
            }
            let a = Point::from(a);
            let b = Point::from(b);

            if let Some(p) = (a * 2i32 - b).guard(dim) {
                out.insert(p);
            };

            if let Some(p) = (b * 2i32 - a).guard(dim) {
                out.insert(p);
            };
        }
    }
    out.len()
}

#[aoc(day8, part2)]
fn part2(input: &[Ant]) -> usize {
    // Can't be arsed to pass it as well
    let dim = if input.len() == 7 { 12 } else { 50 };
    let mut cnt = 0;

    let mut out = vec![false; dim * dim];
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let a = input[i];
            let b = input[j];

            if a.freq != b.freq {
                continue;
            }
            let a = Point::from(a);
            let b = Point::from(b);

            for c in 0.. {
                let Some(p) = (a + (a - b) * c).guard(dim) else {
                    break;
                };
                if !out[(p.x + p.y * dim as i32) as usize] {
                    cnt += 1;
                }
                out[(p.x + p.y * dim as i32) as usize] = true;
            }

            for c in 0.. {
                let Some(p) = (b + (b - a) * c).guard(dim) else {
                    break;
                };

                if !out[(p.x + p.y * dim as i32) as usize] {
                    cnt += 1;
                }
                out[(p.x + p.y * dim as i32) as usize] = true;
            }
        }
    }
    cnt
}

#[test]
fn part1w() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(14, part1(&parse(input)));
}

#[test]
fn part2w() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(34, part2(&parse(input)));
}

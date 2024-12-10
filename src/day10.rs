use std::collections::{HashMap, HashSet};

use crate::{Point, TwoDee};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse(input: &str) -> TwoDee<u8> {
    input
        .lines()
        .map(|c| c.bytes().map(|b| b - b'0'))
        .collect::<TwoDee<u8>>()
}

#[aoc(day10, part1)]
fn part1(input: &TwoDee<u8>) -> usize {
    let accessible = populate(input);
    let mut count = 0;
    for x in 0..input.width {
        for y in 0..input.width {
            if input[(x, y)] == 0 {
                count += accessible[(x, y)].len();
            }
        }
    }
    count
}

fn populate_bfs(input: &TwoDee<u8>) -> TwoDee<HashMap<Point, usize>> {
    let mut accessible: TwoDee<HashMap<Point, usize>> = input.map(|_| HashMap::new());
    let mut current = HashSet::new();
    let mut new = HashSet::new();

    for x in 0..input.width {
        for y in 0..input.width {
            if input[(x, y)] == 9 {
                let p = Point::new(x, y);
                accessible[(x, y)].insert(p, 1);
                current.insert(p);
            }
        }
    }

    for h in [8, 7, 6, 5, 4, 3, 2, 1, 0] {
        //        println!("Looking for points of height {h:?} adjacent to {current:?}");
        for from in current.drain() {
            for dir in &[Point::L, Point::R, Point::U, Point::D] {
                let Some(to) = (*dir + from).guard(input.width) else {
                    continue;
                };

                if input[to] == h {
                    new.insert(to);
                    let (from_s, to_s) = accessible.get_two_mut(from, to).unwrap();
                    for (k, v) in from_s {
                        *to_s.entry(*k).or_default() += *v;
                    }
                }
                if h == 9 {
                    accessible[(x, y)].insert(Point::new(x, y), 1);
                } else {
                    let p = Point::new(x, y);
                    for dir in &[Point::L, Point::R, Point::U, Point::D] {
                        let Some(adj) = (*dir + p).guard(input.width) else {
                            continue;
                        };

                        if input[p] + 1 == input[adj] {
                            let (from, to) = accessible.get_two_mut(adj, p).unwrap();
                            for (k, v) in from {
                                *to.entry(*k).or_default() += *v;
                            }
                        }
                    }
                }
            }
        }
    }
    accessible
}

#[aoc(day10, part2)]
fn part2(input: &TwoDee<u8>) -> usize {
    let accessible = populate(input);
    let mut count = 0;

    for x in 0..input.width {
        for y in 0..input.width {
            if input[(x, y)] == 0 {
                count += accessible[(x, y)].values().sum::<usize>();
            }
        }
    }
    count
}

#[aoc(day10, part2, bfs)]
fn part2_b(input: &TwoDee<u8>) -> usize {
    let accessible = populate_bfs(input);
    let mut count = 0;

    for x in 0..input.width {
        for y in 0..input.width {
            if input[(x, y)] == 0 {
                count += accessible[(x, y)].values().sum::<usize>();
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "0123
1234
8765
9876";
        assert_eq!(part1(&parse(input)), 1);

        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(&parse(input)), 36);
    }

    #[test]
    fn part2_example() {
        let input = "012345
123456
234567
345678
416789
567891";
        assert_eq!(part2(&parse(input)), 227);

        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part2(&parse(input)), 81);
    }
}

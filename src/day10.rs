use std::collections::HashSet;

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
    let mut accessible: TwoDee<HashSet<Point>> = input.map(|_| HashSet::new());
    let mut count = 0;
    for h in [9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
        for x in 0..input.width {
            for y in 0..input.width {
                if input[(x, y)] != h {
                    continue;
                }
                if h == 9 {
                    let mut h = HashSet::new();
                    h.insert(Point::new(x, y));
                    accessible[(x, y)] = h;
                } else {
                    let p = Point::new(x, y);
                    for dir in &[Point::L, Point::R, Point::U, Point::D] {
                        let Some(adj) = (*dir + p).guard(input.width) else {
                            continue;
                        };

                        if input[p] + 1 == input[adj] {
                            // TwoDee - split at mut?
                            let to_add = accessible[adj].clone();
                            accessible[p].extend(to_add.into_iter());
                        }
                    }
                }
            }
        }
    }

    for x in 0..input.width {
        for y in 0..input.width {
            if input[(x, y)] == 0 {
                count += accessible[(x, y)].len();
            }
        }
    }
    count
}

#[aoc(day10, part2)]
fn part2(input: &TwoDee<u8>) -> u32 {
    todo!()
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
        //        assert_eq!(part2(&parse(input)), "<RESULT>");
    }
}

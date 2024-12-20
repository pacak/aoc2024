use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Point, TwoDee};
#[aoc_generator(day20)]
fn parse(input: &str) -> (TwoDee<bool>, Point, Point) {
    let m = input.lines().map(|l| l.bytes()).collect::<TwoDee<u8>>();

    let mut start = None;
    let mut finish = None;
    let maze = m.map_with(|p, b| {
        match b {
            b'S' => start = Some(p),
            b'E' => finish = Some(p),
            _ => {}
        };
        *b == b'#'
    });
    (maze, start.unwrap(), finish.unwrap())
}

fn go1(input: &(TwoDee<bool>, Point, Point), cutoff: usize) -> usize {
    let maze = &input.0;
    let start = input.1;
    let finish = input.2;

    let mut m = maze.map(|_| None);
    m[start] = Some(0usize);
    let mut cur = start;
    let mut steps = 0;
    'outer: while cur != finish {
        for d in Point::DIRS {
            if !maze[cur + d] && m[cur + d].is_none() {
                steps += 1;
                cur = cur + d;
                m[cur] = Some(steps);
                continue 'outer;
            }
        }
    }

    let mut c = 0;
    for x in 0..m.width {
        for y in 0..m.width {
            let cur = Point {
                x: x as i32,
                y: y as i32,
            };

            for d in Point::DIRS {
                if maze.get_point(cur + d * 2).is_none() {
                    continue;
                }
                if maze[cur] || maze[cur + d * 2] {
                    continue;
                }
                let Some(a) = m[cur] else {
                    continue;
                };
                let Some(b) = m[cur + d * 2] else {
                    continue;
                };
                if a + cutoff + 2 <= b {
                    c += 1;
                }
            }
        }
    }
    c
}

fn go2(input: &(TwoDee<bool>, Point, Point), cutoff: usize) -> usize {
    let maze = &input.0;
    let start = input.1;
    let finish = input.2;

    let mut m = maze.map(|_| None);
    m[start] = Some(0usize);
    let mut cur = start;
    let mut steps = 0;
    'outer: while cur != finish {
        for d in Point::DIRS {
            if !maze[cur + d] && m[cur + d].is_none() {
                steps += 1;
                cur = cur + d;
                m[cur] = Some(steps);
                continue 'outer;
            }
        }
    }

    let mut c = 0;
    for x in 0..m.width {
        for y in 0..m.width {
            let cur = Point {
                x: x as i32,
                y: y as i32,
            };

            for n in cur.within(20) {
                if maze.get_point(n).is_none() {
                    continue;
                }
                if maze[cur] || maze[n] {
                    continue;
                }
                let Some(a) = m[cur] else {
                    continue;
                };
                let Some(b) = m[n] else {
                    continue;
                };
                if a + cutoff + cur.distance(n) <= b {
                    c += 1;
                }
            }
        }
    }
    c
}

#[aoc(day20, part1)]
fn part1(input: &(TwoDee<bool>, Point, Point)) -> usize {
    go1(input, 100)
}

#[aoc(day20, part2)]
fn part2(input: &(TwoDee<bool>, Point, Point)) -> usize {
    go2(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(go1(&parse(input), 20), 5);
    }

    #[test]
    fn part2_example() {
        let p = Point { x: 10, y: 10 };

        let input = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(go2(&parse(input), 70), 12 + 22 + 4 + 3);
    }
}

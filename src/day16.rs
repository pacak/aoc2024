use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Point, TwoDee};
#[aoc_generator(day16)]
fn parse(input: &str) -> (TwoDee<bool>, Point, Point) {
    #[derive(PartialEq)]
    enum M {
        Start,
        End,
        Wall,
        Space,
    }
    let twodee = input
        .lines()
        .map(|l| {
            l.bytes().map(|c| match c {
                b'#' => M::Wall,
                b'S' => M::Start,
                b'E' => M::End,
                b'.' => M::Space,
                _ => panic!(),
            })
        })
        .collect::<TwoDee<M>>();
    let mut start = None;
    let mut end = None;
    for x in 0..twodee.width as i32 {
        for y in 0..twodee.width as i32 {
            let p = Point { x, y };
            if twodee[p] == M::Start {
                start = Some(p);
            } else if twodee[p] == M::End {
                end = Some(p);
            }
        }
    }
    (twodee.map(|c| *c == M::Wall), start.unwrap(), end.unwrap())
}
impl Ord for St {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialOrd for St {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct St {
    score: usize,
    cur: Point,
    dir: Point,
    visited: HashSet<Point>,
}
#[aoc(day16, part1)]
fn part1(input: &(TwoDee<bool>, Point, Point)) -> usize {
    let mut maze = input.0.clone();
    let start = input.1;
    let end = input.2;

    let mut q = BinaryHeap::new();
    q.push(St {
        score: 0,
        cur: start,
        dir: Point::R,
        visited: [start].into_iter().collect::<HashSet<_>>(),
    });

    let mut best = HashMap::new();
    while let Some(s) = q.pop() {
        if s.cur == end {
            return s.score;
        }
        for d in [Point::U, Point::R, Point::D, Point::L] {
            if maze[s.cur + d] {
                // wall that way
                continue;
            }

            if s.dir * -1 == d {
                // don't go back
                continue;
            }

            if s.visited.contains(&(s.cur + d)) {
                continue;
            }

            maze.poi = (s.cur.x as usize, s.cur.y as usize);

            let mut s = s.clone();
            s.score += if s.dir == d { 1 } else { 1001 };
            s.cur = s.cur + d;
            s.dir = d;
            s.visited.insert(s.cur);
            match best.get(&s.cur) {
                Some(prev) => {
                    if *prev > s.score {
                        best.insert(s.cur, s.score);
                        q.push(s);
                    }
                }
                None => {
                    best.insert(s.cur, s.score);
                    q.push(s);
                }
            }
        }
    }

    panic!("Where are we?");
}

#[aoc(day16, part2)]
fn part2(input: &(TwoDee<bool>, Point, Point)) -> usize {
    let mut maze = input.0.clone();
    let start = input.1;
    let end = input.2;

    let mut q = BinaryHeap::new();
    q.push(St {
        score: 0,
        cur: start,
        dir: Point::R,
        visited: [start].into_iter().collect::<HashSet<_>>(),
    });

    let mut best = HashMap::<(Point, Point), usize>::new();
    let mut best_path = Vec::<St>::new();
    while let Some(s) = q.pop() {
        if s.cur == end {
            match best_path.last() {
                Some(p) if p.score < s.score => {
                    break;
                }
                _ => {
                    best_path.push(s);
                    continue;
                }
            }
        }

        for d in [Point::U, Point::R, Point::D, Point::L] {
            if maze[s.cur + d] {
                // wall that way
                continue;
            }

            if s.dir * -1 == d {
                // don't go back
                continue;
            }

            if s.visited.contains(&(s.cur + d)) {
                continue;
            }

            maze.poi = (s.cur.x as usize, s.cur.y as usize);

            let mut s = s.clone();
            s.score += if s.dir == d { 1 } else { 1001 };
            s.cur = s.cur + d;
            s.dir = d;
            s.visited.insert(s.cur);
            let t = (s.cur, d);
            let prev = *best.get(&t).unwrap_or(&100000000000000);

            if prev >= s.score {
                if s.score > prev + 1000 {
                    continue;
                }
                if prev >= s.score {
                    best.insert(t, s.score);
                }

                q.push(s);
            }
        }
    }

    best_path
        .iter()
        .flat_map(|b| b.visited.iter())
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part1(&parse(input)), 7036);

        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part1(&parse(input)), 11048);
    }

    #[test]
    fn part2_example() {
        let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part2(&parse(input)), 45);

        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part2(&parse(input)), 64);
    }
}

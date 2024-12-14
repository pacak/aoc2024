use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

use crate::{Point, TwoDee};

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Point,
    vector: Point,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    let reg = Regex::new("^p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)").unwrap();
    let num = |s: &str| s.parse::<i32>().unwrap();
    input
        .lines()
        .map(|l| {
            let line = reg.captures(l).unwrap();
            Robot {
                position: Point {
                    x: num(&line[1]),
                    y: num(&line[2]),
                },
                vector: Point {
                    x: num(&line[3]),
                    y: num(&line[4]),
                },
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    solver(input, 101, 103, 100)
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> usize {
    let mut grid = TwoDee::<bool>::new(110);

    let mut time = 0;
    let width = 101;
    let height = 103;
    loop {
        time += 1;
        grid.data.iter_mut().for_each(|p| *p = false);
        for r in input {
            let x = ((r.position.x + r.vector.x * time) % width + width) % width;
            let y = ((r.position.y + r.vector.y * time) % height + height) % height;

            grid[Point { x, y }] = true;
        }
        let mut t = 0;
        for y in 0..100 {
            if grid[(width as usize / 2, y)] {
                t += 1;
            }
        }
        if t > 20 {
            println!("at {time:?}: {grid:?}");
        }
        if t == 7572 {
            return 7572;
        }
    }
}

fn solver(input: &[Robot], width: i32, height: i32, time: i32) -> usize {
    use std::cmp::Ordering as O;
    let mut qs = [0usize; 4];

    for r in input {
        let x = ((r.position.x + r.vector.x * time) % width + width) % width;
        let y = ((r.position.y + r.vector.y * time) % height + height) % height;

        let q = match (x.cmp(&(width / 2)), y.cmp(&(height / 2))) {
            (O::Less, O::Less) => 1,
            (O::Greater, O::Less) => 2,
            (O::Greater, O::Greater) => 3,
            (O::Less, O::Greater) => 4,
            _ => continue,
        };

        qs[q - 1] += 1;
    }

    qs.into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(solver(&parse(input), 11, 7, 100), 12);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}

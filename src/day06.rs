use std::{cell::Cell, collections::HashSet, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::TwoDee;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Guard {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}
impl Guard {
    fn peek(&self) -> (usize, usize) {
        (
            self.x.wrapping_add_signed(self.dx),
            self.y.wrapping_add_signed(self.dy),
        )
    }

    fn turn(&mut self) {
        (self.dx, self.dy) = match (self.dx, self.dy) {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => panic!(),
        }
    }

    fn advance(&mut self) {
        self.x = self.x.wrapping_add_signed(self.dx);
        self.y = self.y.wrapping_add_signed(self.dy);
    }
}
#[aoc_generator(day6)]
fn parse(input: &str) -> (Guard, TwoDee<bool>) {
    let guard = Rc::new(Cell::new(None));
    let gg = guard.clone();

    let twodee = input
        .lines()
        .enumerate()
        .map(move |(row, l)| {
            let gg = gg.clone();
            l.bytes().enumerate().map(move |(col, b)| {
                if b == b'^' {
                    gg.set(Some(Guard {
                        y: row,
                        x: col,
                        dx: 0,
                        dy: -1,
                    }));
                }

                b == b'#'
            })
        })
        .collect::<crate::TwoDee<bool>>();

    (guard.take().unwrap(), twodee)
}

#[aoc(day6, part1)]
fn part1(input: &(Guard, TwoDee<bool>)) -> usize {
    let (guard_start, room) = input;
    let mut room: TwoDee<bool> = room.clone();
    let mut visited = HashSet::new();
    let mut guard = *guard_start;
    visited.insert((guard.x, guard.y)); // Evil example
    loop {
        let Some(wall) = room.get(guard.peek()).copied() else {
            break;
        };
        room.poi = (guard.x, guard.y);

        if wall {
            guard.turn();
        } else {
            guard.advance();
        }

        visited.insert((guard.x, guard.y));
        if guard == *guard_start {
            break;
        }
    }
    visited.len()
}

#[aoc(day6, part2)]
fn part2(input: &(Guard, TwoDee<bool>)) -> usize {
    let (guard_start, room) = input;
    let mut room: TwoDee<bool> = room.clone();
    let mut visited = HashSet::new();
    let mut guard = *guard_start;
    visited.insert((guard.x, guard.y)); // Evil example

    // first we see here guard visits - adding an obstacle on a cell that was never visited - waste
    // of time
    loop {
        let Some(wall) = room.get(guard.peek()).copied() else {
            break;
        };
        room.poi = (guard.x, guard.y);

        if wall {
            guard.turn();
        } else {
            guard.advance();
        }
        visited.insert((guard.x, guard.y));
    }

    let mut res = 0;
    let mut positions = HashSet::new();
    'next_obstacle: for obstacle in visited.into_iter() {
        positions.clear();
        let mut guard = *guard_start;

        let mut room = room.clone();
        room.poi = guard.peek();
        room[obstacle] = true;

        loop {
            let Some(wall) = room.get(guard.peek()).copied() else {
                continue 'next_obstacle; // out of bounds
            };

            room.poi = guard.peek();

            if wall {
                guard.turn();
            } else {
                guard.advance();
            }

            if !positions.insert(guard) {
                res += 1;
                continue 'next_obstacle;
            }
        }
    }

    res
}

#[test]
fn part1w() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(41, part1(&parse(input)));
}

#[test]
fn part2w() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(6, part2(&parse(input)));
}

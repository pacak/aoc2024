#![allow(dead_code, unused_variables)]
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Point, TwoDee};

impl std::fmt::Debug for TwoDee<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut col = 0;
        let mut row = 0;
        writeln!(f)?;
        for c in self.data.iter() {
            if self.poi == (col, row) {
                write!(f, "X")?;
            } else {
                match c {
                    S::Wall => write!(f, "#"),
                    S::Crate => write!(f, "O"),
                    S::Robot => write!(f, "@"),
                    S::Space => write!(f, "."),
                }?
            }
            col += 1;
            if col == self.width {
                col = 0;
                row += 1;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum S {
    Wall,
    Crate,
    Robot,
    Space,
}

#[aoc_generator(day15)]
fn parse(input: &str) -> (TwoDee<S>, Point, Vec<Point>) {
    let (maze, code) = input.split_once("\n\n").unwrap();
    let maze = maze
        .lines()
        .map(|l| {
            l.chars().map(|c| match c {
                '#' => S::Wall,
                'O' => S::Crate,
                '.' => S::Space,
                '@' => S::Robot,
                _ => panic!(),
            })
        })
        .collect::<TwoDee<S>>();

    let mut robot = None;
    'outer: for x in 0..maze.width as i32 {
        for y in 0..maze.width as i32 {
            let p = Point { x, y };
            if maze[p] == S::Robot {
                robot = Some(p);
                break 'outer;
            }
        }
    }
    let robot = robot.unwrap();
    let prog = code
        .chars()
        .filter_map(|c| {
            Some(match c {
                '<' => Point::L,
                '>' => Point::R,
                '^' => Point::U,
                'v' => Point::D,
                _ => return None,
            })
        })
        .collect::<Vec<_>>();

    (maze, robot, prog)
}

#[aoc(day15, part1)]
fn part1(input: &(TwoDee<S>, Point, Vec<Point>)) -> usize {
    let mut maze = input.0.clone();
    let mut robot = input.1;

    'outer: for dir in input.2.iter().copied() {
        assert_eq!(maze[robot], S::Robot);

        let mut dist = 0;

        loop {
            if maze[robot + dir * dist] == S::Wall {
                continue 'outer;
            } else if maze[robot + dir * dist] == S::Space {
                maze[robot] = S::Space;
                maze[robot + dir * dist] = maze[robot + dir];
                maze[robot + dir] = S::Robot;
                robot = robot + dir;
                continue 'outer;
            } else {
                dist += 1;
            }
        }
    }

    let mut gps = 0;

    for x in 0..maze.width as i32 {
        for y in 0..maze.width as i32 {
            let p = Point { x, y };
            if maze[p] == S::Crate {
                gps += x + y * 100;
            }
        }
    }
    gps as usize
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum SS {
    Wall,
    CrateL,
    CrateR,
    Space,
}

#[aoc(day15, part2)]
fn part2(input: &(TwoDee<S>, Point, Vec<Point>)) -> usize {
    let maze = input.0.clone();
    let mut robot = input.1;
    robot.x *= 2;

    let mut m = HashMap::new();
    for x in 0..maze.width as i32 {
        for y in 0..maze.width as i32 {
            let s = Point { x, y };
            let p = Point { x: x * 2, y };
            match maze[s] {
                S::Wall => {
                    m.insert(p, SS::Wall);
                    m.insert(p.r(), SS::Wall);
                }
                S::Crate => {
                    m.insert(p, SS::CrateL);
                    m.insert(p.r(), SS::CrateR);
                }
                S::Robot | S::Space => {
                    m.insert(p, SS::Space);
                    m.insert(p.r(), SS::Space);
                }
            }
        }
    }
    let mut maze = m;

    'outer: for (ix, dir) in input.2.iter().copied().enumerate() {
        if dir == Point::L || dir == Point::R {
            let mut dist = 1;
            loop {
                if maze[&(robot + dir * dist)] == SS::Wall {
                    continue 'outer;
                } else if maze[&(robot + dir * dist)] == SS::Space {
                    for i in (1..=dist).rev() {
                        swap(&mut maze, robot + dir * i, robot + dir * (i - 1), robot);
                    }
                    robot = robot + dir;
                    continue 'outer;
                } else {
                    dist += 1;
                }
            }
        }
        let mut h = HashSet::new();
        let mut out = Vec::new();
        h.insert(robot);
        if maze[&(robot + dir)] == SS::Space {
            robot = robot + dir;
            continue 'outer;
        } else if maze[&(robot + dir)] == SS::Wall {
            continue 'outer;
        } else if go(&maze, dir, h, &mut out) {
            for p in out.iter().rev().copied() {
                println!("swapping {:?} <-> {:?}", p, p + dir);
                swap(&mut maze, p + dir, p, robot);
            }
            robot = robot + dir;
        }
    }

    // and now something complitely different..
    // At what point do I say screw it, I'm done?

    let mut out = 0;
    for (p, ss) in &maze {
        if *ss == SS::CrateL {
            out += p.x + p.y * 100
        }
    }
    out as usize
}
fn dump(maze: &HashMap<Point, SS>, robot: Point) {
    let w = 10;
    for y in 0..w {
        for x in 0..w * 2 {
            let p = Point { x, y };

            if p == robot {
                print!("@");
            } else {
                match maze[&p] {
                    SS::Wall => print!("#"),
                    SS::CrateL => print!("["),
                    SS::CrateR => print!("]"),
                    SS::Space => print!("."),
                }
            }
        }
        println!();
    }
    println!();
}

fn swap(maze: &mut HashMap<Point, SS>, to: Point, from: Point, robot: Point) {
    assert_eq!(maze[&to], SS::Space);

    let tmp = maze.remove(&from).unwrap();
    maze.insert(from, maze[&to]);
    maze.insert(to, tmp);
}

fn go(
    maze: &HashMap<Point, SS>,
    dir: Point,
    pressure: HashSet<Point>,
    out: &mut Vec<Point>,
) -> bool {
    assert!(!pressure.is_empty());
    let mut pp2 = HashSet::<Point>::new();
    if pressure.iter().any(|p| maze[&(*p + dir)] == SS::Wall) {
        return false;
    }
    for p in pressure {
        match maze[&(p + dir)] {
            SS::Wall => panic!(),
            SS::CrateL => pp2.extend([(p + dir), (p + dir).r()]),
            SS::CrateR => pp2.extend([p + dir, (p + dir).l()]),
            SS::Space => {}
        };
    }
    out.extend(pp2.iter().copied());
    if pp2.is_empty() {
        true
    } else {
        go(maze, dir, pp2, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part1(&parse(input)), 10092);
    }

    #[test]
    fn part2_example() {
        let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part2(&parse(input)), 9021);
    }
}

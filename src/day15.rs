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

// #[aoc(day15, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

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

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}

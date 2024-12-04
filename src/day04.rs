use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

// Vec<Vec
#[aoc_generator(day4)]
fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let mut res = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            res.insert((x as i32, y as i32), c);
        }
    }
    res
}

#[aoc(day4, part1)]
fn part1(input: &HashMap<(i32, i32), char>) -> u32 {
    let mut count = 0;

    for y in 0i32.. {
        if !input.contains_key(&(0, y)) {
            return count;
        }
        for x in 0i32.. {
            if !input.contains_key(&(x, y)) {
                break;
            }

            for dx in [-1i32, 0, 1] {
                'next_cell: for dy in [-1i32, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
                        let i = i as i32;
                        if input.get(&(x + dx * i, y + dy * i)).copied().unwrap_or('?') != *c {
                            continue 'next_cell;
                        }
                    }

                    count += 1;
                }
            }
        }
    }

    // never actually reach here
    count
}

#[aoc(day4, part2)]
fn part2(input: &HashMap<(i32, i32), char>) -> u32 {
    let get = |x: i32, y: i32| input.get(&(x, y)).copied().unwrap_or('?');
    let mut count = 0;

    for y in 0i32.. {
        if !input.contains_key(&(0, y)) {
            return count;
        }
        'next_cell: for x in 0i32.. {
            if !input.contains_key(&(x, y)) {
                break;
            }

            if get(x, y) != 'A' {
                continue;
            }
            for dx in &[-1, 1] {
                for dy in &[-1, 1] {
                    // for each starting cell check 4 possible positions for the main diagonal
                    if !(get(x - dx, y - dy) == 'M' && get(x + dx, y + dy) == 'S') {
                        continue;
                    }

                    let (sx, sy) = match (dx, dy) {
                        (-1, -1) => (1, -1),
                        (1, -1) => (1, 1),
                        (1, 1) => (-1, 1),
                        (-1, 1) => (-1, -1),
                        _ => unreachable!(),
                    };

                    let letters = (get(x - sx, y - sy), get(x + sx, y + sy));
                    if letters == ('M', 'S') || letters == ('S', 'M') {
                        count += 1;
                        continue 'next_cell;
                    }
                }
            }
        }
    }

    // not reaching here
    count
}

#[test]
fn part1_test() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part1(&parse(input)), 18);
}

#[test]
fn part2_test() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part2(&parse(input)), 9);
}

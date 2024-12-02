use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|w| w.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    input.iter().filter(|i| safe1(i)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    input.iter().filter(|i| safe2(i)).count()
}

fn safe1(input: &[u32]) -> bool {
    let dir = input[0].cmp(&input[1]);
    input.windows(2).all(|w| {
        let (a, b) = (w[0], w[1]);
        let diff = if a > b { a - b } else { b - a };
        // The levels are either all increasing or all decreasing.
        // Any two adjacent levels differ by at least one and at most three.
        a.cmp(&b) == dir && (1..=3).contains(&diff)
    })
}

/// if violence is not an answer - you are not using enough of it.
fn safe2(input: &[u32]) -> bool {
    if safe1(input) {
        return true;
    }
    for i in 0..input.len() {
        let mut smaller = input.to_vec();
        smaller.remove(i);
        if safe1(&smaller) {
            return true;
        }
    }
    false
}

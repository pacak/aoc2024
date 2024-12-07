use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.split(&[':', ' '])
                .filter_map(|f| f.parse::<u64>().ok())
                .collect()
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .filter_map(|l| can_be_solved(l).then_some(l[0]))
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .filter_map(|l| can_be_solved2(l).then_some(l[0]))
        .sum()
}

fn can_be_solved(input: &[u64]) -> bool {
    let mut cursor = 1;
    let mut cur = vec![input[cursor]];
    while cursor < input.len() - 1 {
        cursor += 1;
        cur = cur
            .into_iter()
            // list Monad my beloved :)
            .flat_map(|i| [i + input[cursor], i * input[cursor]])
            .collect();
    }
    cur.contains(&input[0])
}

fn pipes(a: u64, b: u64) -> u64 {
    if b == 0 {
        a * 10
    } else {
        a * 10u64.pow(b.ilog10() + 1) + b
    }
}

#[test]
fn pipes_works() {
    assert_eq!(1234, pipes(0, 1234));
    assert_eq!(1234, pipes(1, 234));
    assert_eq!(1234, pipes(123, 4));
    assert_eq!(12340, pipes(1234, 0));
}

fn can_be_solved2(input: &[u64]) -> bool {
    let mut cursor = 1;
    let mut cur = vec![input[cursor]];
    while cursor < input.len() - 1 {
        cursor += 1;
        cur = cur
            .into_iter()
            // list Monad my beloved :)
            .flat_map(|i| {
                [
                    i + input[cursor],
                    i * input[cursor],
                    pipes(i, input[cursor]),
                ]
            })
            .collect();
    }
    cur.contains(&input[0])
}

#[test]
fn part1w() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(part1(&parse(input)), 3749);
}

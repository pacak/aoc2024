use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Block {
    id: Option<u8>,
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Option<usize>> {
    let mut file = true;
    let mut id = 0;
    let mut res = Vec::new();
    for c in input.bytes() {
        let size = c - b'0';

        for _ in 0..size {
            res.push(file.then_some(id));
        }

        if file {
            id += 1;
        }

        file = !file;
    }
    res
}

#[aoc(day9, part1)]
fn part1(input: &[Option<usize>]) -> usize {
    let mut input = input.to_vec();

    let mut i = 0;
    let mut j = input.len() - 1;
    while i < j {
        if input[j].is_none() {
            j -= 1;
        } else if input[i].is_some() {
            i += 1;
        } else {
            input[i] = input[j];
            input[j] = None;
            j -= 1;
            i += 1;
        }
    }
    checksum(&input)
}

fn checksum(input: &[Option<usize>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(ix, mid)| mid.map_or(0, |id| id * ix))
        .sum()
}

#[test]
fn part1w() {
    let input = "2333133121414131402";
    assert_eq!(1928, part1(&parse(input)));
}

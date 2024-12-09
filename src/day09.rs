use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc(day9, part2)]
fn part2(input: &[Option<usize>]) -> usize {
    let mut input = input.to_vec();
    let mut j = input.len() - 1;
    while j > 0 {
        // Keep scanning until the next file
        if input[j].is_none() {
            j -= 1;
            continue;
        }

        let mut jstart = j;
        while jstart > 0 && input[jstart] == input[j] {
            jstart -= 1;
        }

        // Found a file
        let file = jstart + 1..j + 1;

        // found a hole of the right size
        if let Some(hole) = input[..jstart + 1]
            .windows(j - jstart)
            .position(|xs| xs.iter().all(|i| i.is_none()))
        {
            input.copy_within(file.clone(), hole);
            for erase in file {
                input[erase] = None;
            }
        };

        j = jstart;
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

#[test]
fn part2w() {
    let input = "2333133121414131402";
    assert_eq!(2858, part2(&parse(input)));
}

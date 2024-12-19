use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn parse(input: &str) -> (BTreeSet<String>, Vec<String>) {
    let mut lines = input.lines();
    let mut pats = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|m| m.to_owned())
        .collect::<BTreeSet<_>>();

    lines.next();
    let qs = lines.map(|m| m.to_owned()).collect::<Vec<_>>();
    (pats, qs)
}

#[aoc(day19, part1)]
fn part1(input: &(BTreeSet<String>, Vec<String>)) -> usize {
    let (pats, qs) = input;
    let max_pat = pats.iter().map(|l| l.len()).max().unwrap_or_default();

    let mut v = 0;
    for q in qs {
        v += go1(pats, max_pat, q);
    }
    v
}

fn go1(available: &BTreeSet<String>, max: usize, q: &str) -> usize {
    if q.is_empty() {
        return 1;
    }

    for prefix in 1..=max {
        let Some((head, tail)) = q.split_at_checked(prefix) else {
            return 0;
        };

        if available.contains(head) {
            let n = go1(available, max, tail);
            if n > 0 {
                return n;
            }
        }
    }
    0
}

// #[aoc(day19, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_small() {
        let input = "r, wr, b, g, bwu, rb, gb, br\n\nbwurrg";
        assert_eq!(part1(&parse(input)), 1);
    }
    #[test]
    fn part1_example() {
        let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(part1(&parse(input)), 6);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}

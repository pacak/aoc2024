use aoc_runner_derive::{aoc, aoc_generator};

type Rules = Vec<(u32, u32)>;
type Sets = Vec<Vec<u32>>;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Rules, Sets) {
    let (ordering, sets) = input.split_once("\n\n").unwrap();
    let ordering = ordering
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('|').unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    let sets = sets
        .lines()
        .map(|l| {
            l.split(',')
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (ordering, sets)
}

#[aoc(day5, part1)]
fn part1(input: &(Rules, Sets)) -> u32 {
    let mut res = 0;
    for set in input.1.iter() {
        //        let mut set = set.clone();
        if is_valid_sort(set, &input.0) {
            res += set[set.len() / 2];
        }
    }
    res
}

fn is_valid_sort(input: &[u32], rules: &[(u32, u32)]) -> bool {
    for (a, b) in rules {
        if let (Some(pa), Some(pb)) = (
            input.iter().position(|i| i == a),
            input.iter().position(|i| i == b),
        ) {
            if pa > pb {
                return false;
            }
        }
    }
    true
}

#[test]
fn part1w() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part1(&parse(input)), 143);
}

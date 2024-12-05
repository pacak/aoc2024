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
        if is_valid_sort(set, &input.0) {
            res += set[set.len() / 2];
        }
    }
    res
}

#[aoc(day5, part2)]
fn part2(input: &(Rules, Sets)) -> u32 {
    let mut res = 0;
    for set in input.1.iter() {
        if !is_valid_sort(set, &input.0) {
            let mut set = set.clone();
            sort_by_rules(&mut set, &input.0);
            res += set[set.len() / 2];
        }
    }
    res
}

// world's most inefficient sort :)
fn sort_by_rules(input: &mut [u32], rules: &[(u32, u32)]) {
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if rules.iter().any(|r| input[j] == r.0 && input[i] == r.1) {
                input.swap(i, j)
            }
        }
    }
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

#[test]
fn part2w() {
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
    assert_eq!(part2(&parse(input)), 123);
}

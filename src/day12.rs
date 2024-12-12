use std::collections::{BTreeMap, BTreeSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Point, TwoDee};
#[aoc_generator(day12)]
fn parse(input: &str) -> TwoDee<u8> {
    input.lines().map(|l| l.bytes()).collect()
}

// partition grid of items into a grid of unique grid ids
fn uniq_regions(input: &TwoDee<u8>) -> TwoDee<usize> {
    let mut visited = input.map(|_| false);
    let mut res = input.map(|_| usize::MAX);

    let mut pending = BTreeSet::new();
    let mut next_id = 0;
    for x in 0..input.width {
        for y in 0..input.width {
            let p0 = Point::new(x, y);
            if visited[p0] {
                continue;
            }
            visited[p0] = true;
            res[p0] = next_id;
            let cur_region = input[p0];
            pending.extend(p0.adjacent());

            while let Some(p) = pending.pop_first() {
                let Some(p) = p.guard(input.width) else {
                    // out of bounds
                    continue;
                };
                if visited[p] || input[p] != cur_region {
                    // been there already.
                    // Or out of bounds.
                    // Or a different region
                    continue;
                }
                visited[p] = true;
                res[p] = next_id;
                pending.extend(p.adjacent());
            }
            next_id += 1;
        }
    }
    res
}

#[aoc(day12, part1)]
fn part1(input: &TwoDee<u8>) -> usize {
    let u = uniq_regions(input);
    let mut area = BTreeMap::<usize, usize>::new(); // can be a vector...
    let mut perim = BTreeMap::<usize, usize>::new(); // can also be a vector

    for id in &u.data {
        *area.entry(*id).or_default() += 1;
    }
    for x in 0..input.width - 1 {
        for y in 0..input.width {
            let p = Point::new(x, y);
            let cur = u[p];
            let next = u[p + Point::R];
            if cur != next {
                *perim.entry(cur).or_default() += 1;
                *perim.entry(next).or_default() += 1;
            }
        }
    }

    for x in 0..input.width {
        for y in 0..input.width - 1 {
            let p = Point::new(x, y);
            let cur = u[p];
            let next = u[p + Point::D];
            if cur != next {
                *perim.entry(cur).or_default() += 1;
                *perim.entry(next).or_default() += 1;
            }
        }
    }

    for x in 0..input.width {
        *perim.entry(u[Point::new(x, 0)]).or_default() += 1;
        *perim.entry(u[Point::new(x, input.width - 1)]).or_default() += 1;
    }

    for y in 0..input.width {
        *perim.entry(u[Point::new(0, y)]).or_default() += 1;
        *perim.entry(u[Point::new(input.width - 1, y)]).or_default() += 1;
    }

    area.values()
        .zip(perim.values())
        .map(|(a, p)| *a * *p)
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &TwoDee<u8>) -> usize {
    let uniq = uniq_regions(input);
    let mut area = BTreeMap::<usize, usize>::new(); // can be a vector...
    let mut sides = BTreeMap::<usize, usize>::new(); // can also be a vector

    for id in &uniq.data {
        *area.entry(*id).or_default() += 1;
    }

    for x in 0..input.width {
        for y in 0..input.width {
            let p = Point::new(x, y);

            // the same region?
            let has = |tp| uniq.get_point(tp).copied().unwrap_or(usize::MAX) == uniq[p];
            let mut c = 0;

            let u = has(p.u());
            let l = has(p.l());
            let d = has(p.d());
            let r = has(p.r());

            let ul = has(p.u().l());
            let ur = has(p.u().r());
            let dl = has(p.d().l());
            let dr = has(p.d().r());

            // external corner
            c += (!u && !l) as usize;
            c += (!u && !r) as usize;
            c += (!d && !l) as usize;
            c += (!d && !r) as usize;

            // internal corner
            c += (u && l && !ul) as usize;
            c += (u && r && !ur) as usize;
            c += (d && l && !dl) as usize;
            c += (d && r && !dr) as usize;

            *sides.entry(uniq[p]).or_default() += c;
        }
    }

    area.values()
        .zip(sides.values())
        .map(|(a, p)| *a * *p)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
AAA
ABA
ABA";
        assert_eq!(part1(&parse(input)), 7 * 16 + 2 * 6);

        let input = "\
AAA
ABB
AAA";
        assert_eq!(part1(&parse(input)), 7 * 16 + 2 * 6);

        let input = "\
AAA
BBA
AAA";
        assert_eq!(part1(&parse(input)), 7 * 16 + 2 * 6);

        let input = "\
ABA
ABA
AAA";
        assert_eq!(part1(&parse(input)), 7 * 16 + 2 * 6);

        let input = "\
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(&parse(input)), 140);

        let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part1(&parse(input)), 772);

        let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part1(&parse(input)), 1930);
    }

    #[test]
    fn part2_example() {
        let input = "\
AAA
ABB
AAA";
        assert_eq!(part2(&parse(input)), 8 * 7 + 4 * 2);

        let input = "AA\nAA";
        assert_eq!(part2(&parse(input)), 4 * 4);

        let input = "\
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part2(&parse(input)), 80);

        let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part2(&parse(input)), 436);

        let input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(part2(&parse(input)), 236);

        let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(part2(&parse(input)), 368);
    }
}

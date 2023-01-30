use std::cmp::{max, min};
use std::ops::Range;

type Input = Vec<(Range<u8>, Range<u8>)>;

fn cover(v1: &Range<u8>, v2: &Range<u8>) -> bool {
    v2.start >= v1.start && v2.end <= v1.end
}

fn overlaps(v1: &Range<u8>, v2: &Range<u8>) -> bool {
    max(v1.start, v2.start) <= min(v1.end, v2.end)
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (part1, part2) = l.split_once(",").unwrap();
            let (s1, e1) = part1.split_once("-").unwrap();
            let (s2, e2) = part2.split_once("-").unwrap();
            (
                (s1.parse().unwrap()..e1.parse().unwrap()),
                (s2.parse().unwrap()..e2.parse().unwrap()),
            )
        })
        .collect::<Input>()
}

pub fn part_one(input: Input) -> Option<usize> {
    Some(
        input
            .iter()
            .filter(|(v1, v2)| cover(v1, v2) || cover(v2, v1))
            .count(),
    )
}

pub fn part_two(input: Input) -> Option<usize> {
    Some(input.iter().filter(|(v1, v2)| overlaps(v1, v2)).count())
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 4)));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 4)));
        assert_eq!(result, Some(4));
    }
}

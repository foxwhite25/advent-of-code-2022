use itertools::{sorted};

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

pub fn part_one(input: Input) -> Option<u32> {
    input.iter().max().copied()
}

pub fn part_two(input: Input) -> Option<u32> {
    Some(
        sorted(input).rev().take(3).sum()
    )
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 1)));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 1)));
        assert_eq!(result, Some(45000));
    }
}

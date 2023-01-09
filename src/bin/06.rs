use itertools::Itertools;

type Input = Vec<char>;

fn no_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return false;
        }
    }
    true
}

fn first_marker(input: Input, windows_size: usize) -> Option<usize> {
    input
        .windows(windows_size)
        .find_position(|&x| no_dup(x))
        .map(|(pos, _)| pos + windows_size)
}

fn parse(input: &str) -> Input {
    input.trim().chars().collect()
}

pub fn part_one(input: Input) -> Option<usize> {
    first_marker(input, 4)
}

pub fn part_two(input: Input) -> Option<usize> {
    first_marker(input, 14)
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 6)));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 6)));
        assert_eq!(result, Some(19));
    }
}

type Input<'a> = Vec<&'a str>;

fn parse(input: &str) -> Input {
    input.lines().collect()
}

fn priority_func(input: u8) -> u8 {
    input % 32 + (26 * (input <= 90) as u8)
}

pub fn part_one(input: Input) -> Option<u32> {
    Some(
        input
            .iter()
            .filter_map(|&s| {
                let (a, b) = s.split_at(s.len() / 2);
                a.as_bytes()
                    .iter()
                    .find(|byte| b.as_bytes().contains(byte))
                    .map(|&byte| priority_func(byte) as u32)
            })
            .sum(),
    )
}

pub fn part_two(input: Input) -> Option<u32> {
    Some(
        input
            .chunks(3)
            .filter_map(|chunk| {
                let mut k = chunk.iter();
                let a = k.next()?.as_bytes();
                let b = k.next()?.as_bytes();
                let c = k.next()?.as_bytes();

                a.iter()
                    .find(|byte| b.contains(byte) && c.contains(byte))
                    .map(|&byte| priority_func(byte) as u32)
            })
            .sum(),
    )
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 3)));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 3)));
        assert_eq!(result, Some(70));
    }
}

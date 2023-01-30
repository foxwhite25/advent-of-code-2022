type Input = Vec<(i8, i8)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            let byte = l.as_bytes();
            Some((
                (byte.first()? - 'A' as u8) as i8,
                (byte.last()? - 'X' as u8) as i8,
            ))
        })
        .collect()
}

fn result_score(their: i8, mine: i8) -> i8 {
    (3 - (2 + their - mine) % 3) % 3 * 3
}

pub fn part_one(input: Input) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|&(x, y)| (result_score(x, y) + y + 1) as u32)
            .sum(),
    )
}

pub fn part_two(input: Input) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|&(x, y)| {
                let mine = match y {
                    0 => (x + 2) % 3,
                    1 => x,
                    2 => (x + 1) % 3,
                    _ => unreachable!(),
                };
                (result_score(x, mine) + mine + 1) as u32
            })
            .sum(),
    )
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 2)));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 2)));
        assert_eq!(result, Some(12));
    }
}

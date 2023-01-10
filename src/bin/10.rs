use itertools::Itertools;

type Input = Vec<i16>;

fn in_sprite_range(i: usize, x: i16) -> bool {
    (x-1..=x+1).contains(&((i % 40) as i16))
}

fn parse(input: &str) -> Input {
    let mut val = 1;
    let mut k = vec![1];
    let mut other = input
        .trim()
        .split(|c| c == ' ' || c == '\n')
        .filter_map(| x| {
            if x == "" {
                return None
            }
            val += x.parse::<i16>().unwrap_or(0);
            Some(val)
        })
        .collect::<Vec<i16>>();
    k.append(&mut other);
    k
}

pub fn part_one(input: Input) -> Option<i16> {
    Some(
        input
            .iter()
            .skip(19)
            .step_by(40)
            .enumerate()
            .map(|(i, &x)| (i * 40 + 20) as i16 * x)
            .sum()
    )
}

pub fn part_two(input: Input) -> Option<String> {
    let k = input.iter()
        .enumerate()
        .filter_map(|(i, &x)| {
            if i < 240 {
                Some(if in_sprite_range(i, x) { '▓' } else { '░' })
            } else {
                None
            }
        })
        .chunks(40)
        .into_iter()
        .map(|x| x.collect::<String>())
        .join("\n")
        .trim()
        .to_string();
    Some(k)
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 10)));
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let expected = "▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░\n▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░\n▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░\n▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░\n▓▓▓▓▓▓░░░░░░▓▓▓▓▓▓░░░░░░▓▓▓▓▓▓░░░░░░▓▓▓▓\n▓▓▓▓▓▓▓░░░░░░░▓▓▓▓▓▓▓░░░░░░░▓▓▓▓▓▓▓░░░░░";
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 10)));
        assert_eq!(result, Some(expected.trim().to_string()));
    }
}

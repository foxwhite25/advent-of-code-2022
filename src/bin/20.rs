use advent_of_code::treap::Treap;
use itertools::Itertools;

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect_vec()
}

fn decrypt(input: &[i64], multi: i64, k: usize) -> Option<i64> {
    let zero_idx = input.iter().position(|x| *x == 0).unwrap();
    let mut rng = rand::thread_rng();
    let mut trp = Treap::default();
    let mut nodes = input
        .iter()
        .enumerate()
        .map(|(i, n)| trp.insert(*n * multi, i, &mut rng))
        .collect_vec();

    for _ in 0..k {
        for node in &mut nodes {
            let (value, rank) = trp.remove(*node)?;
            let new_rank = (value + rank as i64).rem_euclid(input.len() as i64 - 1);
            *node = trp.insert(value, new_rank as usize, &mut rng);
        }
    }

    let zero_rank = trp.rank(nodes[zero_idx]).unwrap();
    Some(
        (1..=3)
            .map(|k| {
                let rank = trp.derank((zero_rank + 1000 * k) % input.len());
                trp.get(rank).unwrap()
            })
            .sum(),
    )
}

pub fn part_one(input: Input) -> Option<i64> {
    decrypt(&input, 1, 1)
}

pub fn part_two(input: Input) -> Option<i64> {
    decrypt(&input, 811589153, 10)
}

advent_of_code::main!(20);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 20)));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 20)));
        assert_eq!(result, Some(1623178306));
    }
}

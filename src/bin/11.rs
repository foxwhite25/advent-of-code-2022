type Input = Vec<Monkey>;
// type Input = Vec<u32>;

#[derive(Clone, Debug)]
pub enum Op {
    Add(u64),
    Multi(u64),
    MultiSelf,
}

impl Op {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Self::Add(k) => x + k,
            Self::Multi(k) => x * k,
            Self::MultiSelf => x * x,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    init_item: Vec<u64>,
    operation: Op,
    divide_by: u64,
    if_true: usize,
    if_false: usize,
}

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .filter_map(|chunk| {
            let pieces: Vec<&str> = chunk.split(|x| x == ':' || x == '\n').map(|x| x.trim()).collect();
            if pieces.len() < 11 {
                return None;
            }

            let op = pieces[5].strip_prefix("new = old ").unwrap().split_once(" ").unwrap();

            Some(
                Monkey {
                    init_item: pieces[3]
                        .split(", ")
                        .map(|x| x.parse().unwrap())
                        .collect(),
                    operation: match op {
                        ("+", arg) => Op::Add(arg.parse().unwrap()),
                        ("*", "old") => Op::MultiSelf,
                        ("*", arg) => Op::Multi(arg.parse().unwrap()),
                        _ => unreachable!()
                    },
                    divide_by: pieces[7].strip_prefix("divisible by ").unwrap().parse().unwrap(),
                    if_true: pieces[9].strip_prefix("throw to monkey ").unwrap().parse().unwrap(),
                    if_false: pieces[11].strip_prefix("throw to monkey ").unwrap().parse().unwrap(),
                }
            )
        })
        .collect()
}

fn simulate(monkeys: Input, round: usize, part_one: bool) -> usize {
    let mut inspection = vec![0; monkeys.len()];
    let mut items = monkeys.iter().map(|x| x.init_item.clone()).collect::<Vec<_>>();
    let base: u64 = monkeys.iter().map(|m| m.divide_by).product();

    (0..round)
        .for_each(|_| {
            monkeys
                .iter()
                .enumerate()
                .for_each(|(i, monkey)| {
                    let item = items[i]
                        .drain(..)
                        .map(|x| {
                            if part_one { monkey.operation.apply(x) / 3 } else { monkey.operation.apply(x) % base }
                        })
                        .collect::<Vec<_>>();

                    inspection[i] += item.len();

                    for x in item {
                        if x % monkey.divide_by == 0 {
                            items[monkey.if_true].push(x);
                        } else {
                            items[monkey.if_false].push(x);
                        }
                    }
                })
        });

    inspection.sort_unstable();
    inspection[inspection.len() - 1] * inspection[inspection.len() - 2]
}

pub fn part_one(input: Input) -> Option<usize> {
    Some(simulate(input, 20, true))
}

pub fn part_two(input: Input) -> Option<usize> {
    Some(simulate(input, 10000, false))
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 11)));
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 11)));
        assert_eq!(result, Some(2713310158));
    }
}

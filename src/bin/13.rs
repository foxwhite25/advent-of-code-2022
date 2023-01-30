use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub enum Packet {
    Number(u32),
    Array(Vec<Packet>),
}

type Input = Vec<Packet>;

impl Packet {
    fn parse(input: &str) -> Option<Packet> {
        let mut packet = Vec::new();
        let mut chars = input.chars();
        let mut current = chars.next()?;
        let mut is_array = false;
        if current == '[' {
            is_array = true;
            current = chars.next()?;
        }

        while current != ']' {
            if current == ',' {
                current = chars.next()?;
            }

            if current.is_digit(10) {
                let mut number = String::new();
                while current.is_digit(10) {
                    number.push(current);
                    current = chars.next()?;
                }
                packet.push(Packet::Number(number.parse().unwrap()));
            }

            if current == '[' {
                let mut array = String::new();
                let mut depth = 1;
                while depth > 0 {
                    array.push(current);
                    current = chars.next()?;
                    if current == '[' {
                        depth += 1;
                    }
                    if current == ']' {
                        depth -= 1;
                    }
                }
                array.push(current);
                packet.push(Packet::parse(&array)?);
                current = chars.next()?;
            }
        }

        if is_array {
            Some(Packet::Array(packet))
        } else {
            packet.pop()
        }
    }

    fn right_order(packet_a: &Packet, packet_b: &Packet) -> Option<bool> {
        match (packet_a, packet_b) {
            (Packet::Number(a), Packet::Number(b)) => {
                if a == b {
                    return None;
                }
                Some(a < b)
            }
            (Packet::Array(a), Packet::Array(b)) => {
                let mut left = a.iter();
                let mut right = b.iter();

                loop {
                    match (left.next(), right.next()) {
                        (None, None) => return None,
                        (None, Some(_)) => return Some(true),
                        (Some(_), None) => return Some(false),
                        (Some(a), Some(b)) => match Packet::right_order(a, b) {
                            None => continue,
                            k => return k,
                        },
                    }
                }
            }
            (Packet::Number(a), Packet::Array(_)) => {
                let a = Packet::Array(vec![Packet::Number(*a)]);
                Packet::right_order(&a, packet_b)
            }
            (Packet::Array(_), Packet::Number(b)) => {
                let b = Packet::Array(vec![Packet::Number(*b)]);
                Packet::right_order(packet_a, &b)
            }
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{}", n),
            Packet::Array(a) => {
                write!(f, "[")?;
                for (i, p) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", p)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match Packet::right_order(self, other) {
            None => None,
            Some(true) => Some(Ordering::Less),
            Some(false) => Some(Ordering::Greater),
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match Packet::right_order(self, other) {
            None => Ordering::Equal,
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
        }
    }
}

fn parse(input: &str) -> Input {
    input.lines().filter_map(Packet::parse).collect()
}

pub fn part_one(input: Input) -> Option<usize> {
    Some(
        input
            .chunks(2)
            .enumerate()
            .filter_map(|(i, x)| {
                if Packet::right_order(&x[0], &x[1]).unwrap() {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(mut input: Input) -> Option<usize> {
    let divider_a = Packet::Array(vec![Packet::Number(2)]);
    let divider_b = Packet::Array(vec![Packet::Number(6)]);

    input.push(divider_a.clone());
    input.push(divider_b.clone());

    input.sort();
    Some(
        input
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if x == &divider_a {
                    Some(i + 1)
                } else if x == &divider_b {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .product(),
    )
}

advent_of_code::main!(13);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 13)));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 13)));
        assert_eq!(result, Some(140));
    }
}

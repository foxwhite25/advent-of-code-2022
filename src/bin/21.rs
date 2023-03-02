use std::collections::HashMap;

type MonkeyId = u32;

static ROOT_ID: MonkeyId = 0x746F6F72;
static HUMN_ID: MonkeyId = 0x6E6D7568;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn perform_op(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Monkey {
    Yell(i64),
    Parent(Box<(Monkey, Op, Monkey)>),
    Human(i64),
}

impl Monkey {
    fn from_str(id: MonkeyId, known_values: &HashMap<MonkeyId, String>) -> Self {
        let s = known_values.get(&id).unwrap();
        if s.chars().filter(|c| c.is_numeric()).count() > 0 {
            let n = s.parse::<i64>().unwrap();
            if id == HUMN_ID {
                Monkey::Human(n)
            } else {
                Monkey::Yell(n)
            }
        } else {
            let mut args = s.splitn(3, ' ');
            let lhs = Monkey::from_str(str_id_to_monkey_id(args.next().unwrap()), known_values);
            let op = match args.next().unwrap() {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => panic!("\"{}\" is not a valid math operator", args.next().unwrap()),
            };
            let rhs = Monkey::from_str(str_id_to_monkey_id(args.next().unwrap()), known_values);
            Monkey::Parent(Box::new((lhs, op, rhs)))
        }
    }

    fn into_children(self) -> (Monkey, Op, Monkey) {
        if let Monkey::Parent(b) = self {
            *b
        } else {
            panic!("attempted to get children of a non-parent")
        }
    }

    fn value(&self) -> i64 {
        match self {
            Monkey::Yell(n) | Monkey::Human(n) => *n,
            Monkey::Parent(b) => {
                let (lhs, op, rhs) = b.as_ref();
                op.perform_op(lhs.value(), rhs.value())
            }
        }
    }

    fn make_constant(&mut self) {
        if let Monkey::Parent(b) = self {
            let (lhs, op, rhs) = b.as_mut();
            lhs.make_constant();
            rhs.make_constant();
            if let (Monkey::Yell(a), Monkey::Yell(b)) = (lhs, rhs) {
                *self = Monkey::Yell(op.perform_op(*a, *b));
            }
        }
    }

    fn undo_op(self, h: &mut i64) -> Monkey {
        let (lhs, op, rhs) = self.into_children();
        match op {
            Op::Add => {
                let (cons, var) = if matches!(lhs, Monkey::Yell(..)) {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                *h -= cons.value();
                var
            }
            Op::Sub => {
                if let Monkey::Yell(n) = lhs {
                    *h = n - *h;
                    rhs
                } else if let Monkey::Yell(n) = rhs {
                    *h += n;
                    lhs
                } else {
                    unreachable!()
                }
            }
            Op::Mul => {
                let (cons, var) = if matches!(lhs, Monkey::Yell(..)) {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                *h /= cons.value();
                var
            }
            Op::Div => {
                if let Monkey::Yell(n) = lhs {
                    *h = n / *h;
                    rhs
                } else if let Monkey::Yell(n) = rhs {
                    *h *= n;
                    lhs
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn solve_for_humn(mut self) -> i64 {
        self.make_constant();
        let (lhs, _, rhs) = self.into_children();
        let (constant_side, mut human_side) = if matches!(lhs, Monkey::Yell(..)) {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };
        let mut h = constant_side.value();
        while !matches!(human_side, Monkey::Human(..)) {
            human_side = human_side.undo_op(&mut h);
        }
        h
    }
}

type Input = Monkey;

fn parse(input: &str) -> Input {
    let known_values = input
        .lines()
        .filter_map(|line| {
            let (id, yell_str) = line.split_once(": ")?;
            Some((str_id_to_monkey_id(id), yell_str.to_string()))
        })
        .collect();

    Monkey::from_str(ROOT_ID, &known_values)
}

fn str_id_to_monkey_id(id: &str) -> MonkeyId {
    let id = id.as_bytes();
    u32::from_le_bytes([id[0], id[1], id[2], id[3]])
}

pub fn part_one(input: Input) -> Option<i64> {
    Some(input.value())
}

pub fn part_two(input: Input) -> Option<i64> {
    Some(input.solve_for_humn())
}

advent_of_code::main!(21);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 21)));
        assert_eq!(result, Some(152));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 21)));
        assert_eq!(result, Some(301));
    }
}

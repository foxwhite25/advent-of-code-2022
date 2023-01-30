use itertools::Itertools;

type Input = (Stack, Vec<Instruction>);

type Stack = Vec<Vec<char>>;
type Instruction = (usize, usize, usize);

fn move_stacks(stack: &mut Stack, instruction: Vec<Instruction>, grouped: bool) {
    instruction.iter().for_each(|&(count, from, to)| {
        let pillar = &mut stack[from - 1];
        let boxes = pillar.split_off(pillar.len() - count);
        if grouped {
            stack[to - 1].extend(boxes.iter())
        } else {
            stack[to - 1].extend(boxes.iter().rev())
        }
    })
}

fn top_row_string(stack: Stack) -> String {
    stack.iter().filter_map(|pillar| pillar.last()).join("")
}

fn parse(input: &str) -> Input {
    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();
    let mut stack_iter = stack_input.lines().rev();
    let mut stack = vec![vec![]; stack_iter.next().unwrap().len() / 4 + 1];

    stack_iter.for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, x)| if x != ' ' { stack[i].push(x) })
    });

    let instructions = instruction_input
        .lines()
        .map(|x| {
            let k = x.split_ascii_whitespace().collect::<Vec<&str>>();
            (k[1].parse().unwrap(), k[3].parse().unwrap(), k[5].parse().unwrap())
        }).collect();
    (stack, instructions)
}

pub fn part_one(input: Input) -> Option<String> {
    let (mut stack, instruction) = input;
    move_stacks(&mut stack, instruction, false);
    Some(
        top_row_string(stack)
    )
}

pub fn part_two(input: Input) -> Option<String> {
    let (mut stack, instruction) = input;
    move_stacks(&mut stack, instruction, true);
    Some(
        top_row_string(stack)
    )
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 5)));
        assert_eq!(result, Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 5)));
        assert_eq!(result, Some("MCD".to_string()));
    }
}

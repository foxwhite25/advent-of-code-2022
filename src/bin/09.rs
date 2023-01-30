use std::cmp::{max, min};

use advent_of_code::{Direction, Point, SparseGrid};

type Input = Vec<(Direction, u32)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (dir, space) = line.split_once(" ").unwrap();
            (
                match dir {
                    "U" => Direction::North,
                    "R" => Direction::East,
                    "D" => Direction::South,
                    "L" => Direction::West,
                    _ => unreachable!(),
                },
                space.parse::<u32>().unwrap(),
            )
        })
        .collect::<Input>()
}

fn knot_pos(head: &Point, knot: &Point) -> Point {
    Point {
        x: knot.x + max(min(head.x - knot.x, 1), -1),
        y: knot.y + max(min(head.y - knot.y, 1), -1),
    }
}

fn simulate_knot(steps: Input, knot_length: usize) -> usize {
    let mut knots = (0..knot_length)
        .map(|_| Point { x: 0, y: 0 })
        .collect::<Vec<Point>>();
    let mut grid = SparseGrid::default();

    grid.insert(Point { x: 0, y: 0 }, ());

    steps.iter().for_each(|(dir, length)| {
        (0..*length).for_each(|_| {
            knots[0] = knots[0].get_neighbour(dir, 1);
            (0..(knots.len() - 1)).for_each(|i| {
                let head = &knots[i];
                let tail = &knots[i + 1];

                if head.chebyshev_distance(tail) > 1 {
                    knots[i + 1] = knot_pos(head, tail);
                    if i == knot_length - 2 {
                        grid.insert(knots[i + 1].clone(), ())
                    }
                }
            })
        })
    });

    grid.points.len()
}

pub fn part_one(input: Input) -> Option<usize> {
    Some(simulate_knot(input, 2))
}

pub fn part_two(input: Input) -> Option<usize> {
    Some(simulate_knot(input, 10))
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(
            &advent_of_code::template::read_file("examples", 9)
                .split_once("\n\n")
                .unwrap()
                .0,
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(
            &advent_of_code::template::read_file("examples", 9)
                .split_once("\n\n")
                .unwrap()
                .1,
        ));
        assert_eq!(result, Some(36));
    }
}

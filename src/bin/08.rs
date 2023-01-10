use itertools::Itertools;
use advent_of_code::{Direction, Point, SimpleGrid};

type Input<'a> = SimpleGrid<u32>;

static FOUR_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn parse(input: &str) -> Input {
    SimpleGrid::from_str(input, &mut |c, _, _| c.to_digit(10).unwrap())
}

pub fn part_one(input: Input) -> Option<usize> {
    Some(
        input
            .points()
            .iter()
            .filter(|&point| {
                if input.is_boundary(point) {
                    true
                } else {
                    let height = input.get(point);
                    FOUR_DIRECTIONS
                        .iter()
                        .any(|dir| {
                            input
                                .walk(point, dir)
                                .all(|point_b| {
                                    input.get(&point_b) < height
                                })
                        })
                }
            })
            .count()
    )
}

pub fn part_two(input: Input) -> Option<u32> {
    input
        .points()
        .iter()
        .map(|point| {
            let height = input.get(point);

            FOUR_DIRECTIONS
                .iter()
                .map(|dir| {
                    let points = input.walk(point, &dir).collect::<Vec<Point>>();
                    let length = points.len() as u32;

                    points
                        .iter()
                        .find_position(|point_b| input.get(&point_b) >= height)
                        .map(|(pos, _item)| (pos + 1) as u32)
                        .unwrap_or(length)
                })
                .product()
        })
        .max()
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 8)));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 8)));
        assert_eq!(result, Some(8));
    }
}

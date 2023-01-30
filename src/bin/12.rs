use advent_of_code::shortest_path::shortest_path;
use advent_of_code::{Point, SimpleGrid};

type Input = (SimpleGrid<char>, Point, Point);

fn parse(input: &str) -> Input {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let grid: SimpleGrid<char> = SimpleGrid::from_str(input, &mut |c, x, y| match c {
        'S' => {
            start = Some(Point {
                x: x as isize,
                y: y as isize,
            });
            'a'
        }
        'E' => {
            end = Some(Point {
                x: x as isize,
                y: y as isize,
            });
            'z'
        }
        c => c,
    });

    (grid, start.unwrap(), end.unwrap())
}

pub fn part_one(input: Input) -> Option<usize> {
    let (grid, start, end) = input;
    shortest_path(
        &grid,
        &vec![start],
        &end,
        |_| 1,
        |a, b| (*grid.get(b) as isize - *grid.get(a) as isize) < 2,
    )
}

pub fn part_two(input: Input) -> Option<usize> {
    let (grid, _start, end) = input;
    let start_points: Vec<Point> = grid
        .points()
        .into_iter()
        .filter(|point| {
            *grid.get(point) == 'a'
                && grid
                    .cardianal_neighbours(point)
                    .iter()
                    .any(|c| *grid.get(c) == 'b')
        })
        .collect();
    shortest_path(
        &grid,
        &start_points,
        &end,
        |_| 1,
        |a, b| (*grid.get(b) as isize - *grid.get(a) as isize) < 2,
    )
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 12)));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 12)));
        assert_eq!(result, Some(29));
    }
}

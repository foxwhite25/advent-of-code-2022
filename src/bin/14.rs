use itertools::Itertools;

use advent_of_code::{Direction, Point, SparseGrid};

#[derive(Clone, Eq, PartialEq)]
pub enum BlockerType {
    Sand,
    Wall,
}

type Input = (SparseGrid<BlockerType>, isize);

fn parse(input: &str) -> Input {
    input
        .lines()
        .fold((SparseGrid::default(), 0), |mut acc, l| {
            l.split(" -> ")
                .filter_map(|segment| {
                    let (x, y) = segment.split_once(',').unwrap();
                    Some(Point {
                        x: x.parse().ok()?,
                        y: y.parse().ok()?,
                    })
                })
                .tuple_windows()
                .for_each(|(a, b)| {
                    for point in a.line_to(&b) {
                        acc.1 = acc.1.max(point.y);
                        acc.0.insert(point, BlockerType::Wall);
                    }
                });
            acc
        })
}

fn next_pos(grid: &SparseGrid<BlockerType>, point: &Point) -> Option<Point> {
    let south = point.get_neighbour(&Direction::South, 1);
    if grid.get(&south).is_none() {
        return Some(south);
    }

    let south_east = point.get_neighbour(&Direction::SouthEast, 1);
    if grid.get(&south_east).is_none() {
        return Some(south_east);
    }

    let south_west = point.get_neighbour(&Direction::SouthWest, 1);
    if grid.get(&south_west).is_none() {
        return Some(south_west);
    }

    None
}

pub fn part_one(mut input: Input) -> Option<u32> {
    let mut path = vec![Point { x: 500, y: 0 }];
    let mut count = 0;

    loop {
        let current_pos = path.last().unwrap();

        match next_pos(&input.0, current_pos) {
            Some(next_pos) => {
                if current_pos.y >= input.1 {
                    break;
                } else {
                    path.push(next_pos);
                }
            }
            None => {
                input.0.insert(current_pos.clone(), BlockerType::Sand);
                path.pop();
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(mut input: Input) -> Option<u32> {
    let mut path = vec![Point { x: 500, y: 0 }];
    let floor = input.1 + 2;
    let mut count = 0;

    loop {
        let current_pos = path.last().unwrap();

        if let Some(next_pos) = next_pos(&input.0, current_pos) {
            if next_pos.y < floor {
                path.push(next_pos);
                continue;
            }
        }
        input.0.insert(current_pos.clone(), BlockerType::Sand);
        count += 1;

        if *current_pos == (Point { x: 500, y: 0 }) {
            break;
        }

        path.pop();
    }
    Some(count)
}

// fn grid_string(grid: &SparseGrid<BlockerType>) -> String {
//     let (min_x, max_x, min_y, max_y) = grid
//         .points
//         .keys()
//         .fold((999999, 0, 999999, 0), |(min_x, max_x, min_y, max_y), point| {
//             (
//                 min(min_x, point.x),
//                 max(max_x, point.x),
//                 min(min_y, point.y),
//                 max(max_y, point.y),
//             )
//         });
//
//     let mut result = String::new();
//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             let point = Point { x, y };
//             result.push(match grid.get(&point) {
//                 Some(BlockerType::Sand) => '░',
//                 Some(BlockerType::Wall) => '█',
//                 None => ' ',
//             });
//         }
//         result.push('\n');
//     }
//     result
// }

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 14)));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 14)));
        assert_eq!(result, Some(93));
    }
}

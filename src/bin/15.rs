use itertools::Itertools;
use advent_of_code::{Point};
use advent_of_code::quadrant::Quadrant;
use advent_of_code::range::{Range, RangeStack};

#[derive(Clone, Debug)]
pub struct Pair {
    sensor: Point,
    beacon: Point,
    distance: isize,
}

impl Pair {
    pub fn can_contain_unseen_points(&self, quadrant: &Quadrant) -> bool {
         quadrant.corners().iter().any(|corner| {
             let distance = self.sensor.manhattan_distance(corner);
             distance > self.distance
         })
    }
}

type Input = Vec<Pair>;

fn find_unseen_points(pairs: &[Pair], quadrant: &Quadrant) -> Option<Point> {
    if quadrant.min == quadrant.max {
        return Some(quadrant.min.clone());
    }

    quadrant
        .subdivide()
        .iter()
        .filter(|&sub| sub.min.x <= sub.max.x && sub.min.y <= sub.max.y)
        .filter(|&sub| pairs.iter().all(|pair| pair.can_contain_unseen_points(sub)))
        .filter_map(|sub| find_unseen_points(pairs, sub))
        .next()
}

fn parse_point(s: &str) -> Option<Point> {
    let (_, point_str) = s.split_once("at ")?;
    let (x_str, y_str) = point_str.split_once(", ")?;
    let (_, x) = x_str.split_once('=')?;
    let (_, y) = y_str.split_once('=')?;
    Some(Point {
        x: x.parse().ok()?,
        y: y.parse().ok()?,
    })
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            let (sensor_str, beacon_str) = l.split_once(':')?;
            let sensor = parse_point(sensor_str)?;
            let beacon = parse_point(beacon_str)?;
            let distance = sensor.manhattan_distance(&beacon);
            Some(
                Pair{sensor, beacon, distance}
            )
        })
        .collect()
}

pub fn part_one(input: Input) -> Option<usize> {
    let test_value = if cfg!(test) { 10 } else { 2000000 };
    let ranges = input
        .iter()
        .filter_map(|pair| {
            let radius = pair.sensor.manhattan_distance(&pair.beacon);
            let radius_at_test = radius - (test_value - pair.sensor.y).abs();

            let min = pair.sensor.x - radius_at_test;
            let max = pair.sensor.x + radius_at_test;
            if min > max {
                None
            } else {
                Some(Range::new(min, max))
            }
        })
        .collect::<RangeStack>();

    Some(
        ranges.count() - input.iter().filter_map(|pair| {
            if pair.beacon.y == test_value {
                Some(pair.beacon.x)
            } else {
                None
            }
        }).unique().count()
    )
}

pub fn part_two(input: Input) -> Option<isize> {
    let unseen_point = find_unseen_points(&input, &Quadrant {
        min: Point { x: 0, y: 0 },
        max: Point { x: 4000000, y: 4000000 },
    })?;
    Some(unseen_point.x*4000000+unseen_point.y)
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 15)));
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 15)));
        assert_eq!(result, Some(56000011));
    }
}

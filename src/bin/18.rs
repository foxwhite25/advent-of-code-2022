use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Cube {
    pub pos: u16, // Pos is a bit map of x, y, z, where each data point is 5 bits and a block is 15 bits
}

impl Cube {
    pub fn neighbours(&self) -> Vec<Cube> {
        let mut neighbours = Vec::new();
        let (x, y, z) = self.pos();
        if x > 0 {
            neighbours.push(Cube { pos: self.pos - 1 });
        }
        if x < 22 {
            neighbours.push(Cube { pos: self.pos + 1 });
        }
        if y > 0 {
            neighbours.push(Cube { pos: self.pos - 32 });
        }
        if y < 22 {
            neighbours.push(Cube { pos: self.pos + 32 });
        }
        if z > 0 {
            neighbours.push(Cube {
                pos: self.pos - 1024,
            });
        }
        if z < 22 {
            neighbours.push(Cube {
                pos: self.pos + 1024,
            });
        }
        neighbours
    }

    pub fn pos(&self) -> (u16, u16, u16) {
        (
            self.pos & 0x1F,
            (self.pos >> 5) & 0x1F,
            (self.pos >> 10) & 0x1F,
        )
    }

    pub fn from_pos(x: u16, y: u16, z: u16) -> Self {
        Self {
            pos: x | (y << 5) | (z << 10),
        }
    }
}

type Input = (Vec<Cube>, HashMap<u16, bool>, usize);

fn parse(input: &str) -> Input {
    let cubes = input
        .lines()
        .filter_map(|line| {
            let mut line_iter = line.split(",");
            let x = line_iter.next()?.parse::<u16>().unwrap();
            let y = line_iter.next()?.parse::<u16>().unwrap();
            let z = line_iter.next()?.parse::<u16>().unwrap();
            Some(Cube::from_pos(x, y, z))
        })
        .collect_vec();
    let mut cube_space_map = HashMap::new();
    let mut zero_count = 0;
    cubes.iter().for_each(|cube| {
        cube_space_map.insert(cube.pos, true);
        let (x, y, z) = cube.pos();
        if x == 0 || y == 0 || z == 0 {
            zero_count += 1;
        }
    });
    (cubes, cube_space_map, zero_count)
}

pub fn part_one(input: Input) -> Option<usize> {
    let (input, cube_space_map, zero_count) = input;

    Some(
        input
            .iter()
            .map(|cube| {
                cube.neighbours()
                    .iter()
                    .filter(|neighbour| cube_space_map.get(&neighbour.pos).is_none())
                    .count()
            })
            .sum::<usize>()
            + zero_count,
    )
}

pub fn part_two(input: Input) -> Option<usize> {
    let mut processed_map = HashMap::new();
    let mut queue = vec![Cube::from_pos(0, 0, 0)];
    let mut exterior = 0;
    let (_, cube_space_map, zero_count) = input;
    while let Some(cube) = queue.pop() {
        exterior += cube
            .neighbours()
            .iter()
            .filter_map(|neighbour| {
                if cube_space_map.get(&neighbour.pos).is_none() {
                    if processed_map.get(&neighbour.pos).is_none() {
                        processed_map.insert(neighbour.pos, true);
                        queue.push(*neighbour);
                    }
                    None
                } else {
                    Some(1)
                }
            })
            .sum::<usize>()
    }
    Some(exterior + zero_count)
}

advent_of_code::main!(18);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 18)));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 18)));
        assert_eq!(result, Some(58));
    }
}

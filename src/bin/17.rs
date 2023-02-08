use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, Copy)]
pub enum Wind {
    Left,
    Right,
}

type Input = Vec<Wind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shape(u32);

impl Shape {
    const fn all_shapes() -> [Self; 5] {
        [
            Self(0x0000001E),
            Self(0x00081C08),
            Self(0x0004041C),
            Self(0x10101010),
            Self(0x00001818),
        ]
    }

    pub fn blow(&mut self, direction: Wind, mask: u32) {
        let new_pos = match direction {
            Wind::Left => {
                if self.0 & 0x40404040 == 0 {
                    self.0 << 1
                } else {
                    return;
                }
            }
            Wind::Right => {
                if self.0 & 0x01010101 == 0 {
                    self.0 >> 1
                } else {
                    return;
                }
            }
        };

        if new_pos & mask == 0 {
            self.0 = new_pos;
        }
    }

    pub const fn intersects(&self, mask: u32) -> bool {
        self.0 & mask != 0
    }

    pub fn as_bytes(self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b != 0)
    }
}

fn tower_mask(tower: &[u8], height: usize) -> u32 {
    if height >= tower.len() {
        0
    } else {
        tower[height..]
            .iter()
            .take(4)
            .rev()
            .fold(0u32, |acc, b| (acc << 8) | *b as u32)
    }
}

fn drop_rock(
    tower: &mut Vec<u8>,
    wind: Vec<Wind>,
    mut wind_idx: usize,
    mut shape: Shape,
) -> Option<usize> {
    let mut height = tower.len() + 3;

    loop {
        let wind_dir = wind[wind_idx];
        wind_idx += 1;
        if wind_idx == wind.len() {
            wind_idx = 0;
        }

        let current_mask = tower_mask(tower, height);

        shape.blow(wind_dir, current_mask);

        if height > tower.len() {
            height -= 1;
        } else if height == 0 || shape.intersects(tower_mask(tower, height - 1)) {
            for byte in shape.as_bytes() {
                if height < tower.len() {
                    tower[height] |= byte;
                } else {
                    tower.push(byte);
                }
                height += 1;
            }
            return Some(wind_idx);
        } else {
            height -= 1;
        }
    }
}

fn parse(input: &str) -> Input {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Wind::Left),
            '>' => Some(Wind::Right),
            _ => None,
        })
        .collect()
}

pub fn part_one(input: Input) -> Option<usize> {
    let num_rocks = 2022;
    let mut tower = Vec::with_capacity(num_rocks * 4);

    let mut wind_idx = 0;
    for shape in Shape::all_shapes().into_iter().cycle().take(num_rocks) {
        wind_idx = drop_rock(&mut tower, input.clone(), wind_idx, shape)?;
    }

    Some(tower.len())
}

pub fn part_two(input: Input) -> Option<usize> {
    let num_rocks = 1000000000000;
    let mut seen_states = HashMap::with_capacity(1024);
    let mut tower = Vec::with_capacity(1024);

    let mut cycle_height = 0;
    let mut wind_idx = 0;
    let shapes = Shape::all_shapes();
    let mut n = 0;
    while n < num_rocks {
        let shape_idx = n % shapes.len();
        let shape = shapes[shape_idx];

        wind_idx = drop_rock(&mut tower, input.clone(), wind_idx, shape)?;
        n += 1;

        if tower.len() < 8 {
            continue;
        }

        let skyline = u64::from_ne_bytes(tower[tower.len() - 8..].try_into().ok()?);
        let state = (skyline, shape_idx, wind_idx);

        match seen_states.entry(state) {
            Entry::Occupied(e) => {
                let (old_n, old_height) = e.get();
                let num_rocks_in_cycle = n - old_n;
                let num_cycles = (num_rocks - n) / num_rocks_in_cycle;
                n += num_rocks_in_cycle * num_cycles;
                cycle_height += num_cycles * (tower.len() - old_height);
                seen_states.clear();
            }
            Entry::Vacant(e) => {
                e.insert((n, tower.len()));
            }
        }
    }

    Some(tower.len() + cycle_height)
}

advent_of_code::main!(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 17)));
        assert_eq!(result, Some(3068));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 17)));
        assert_eq!(result, Some(1514285714288));
    }
}

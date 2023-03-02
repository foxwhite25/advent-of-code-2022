use crate::Material::{Clay, Geode, Obsidian, Ore};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::max;

pub type RecipePart = (u32, Material);

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Clone, Debug)]
pub struct Blueprint {
    id: u32,
    robot_recipes: [Vec<RecipePart>; 4],
}

#[derive(Copy, Clone)]
pub struct SearchState {
    time_remaining: u32,
    robots: [u32; 4],
    materials: [u32; 4],
}

impl SearchState {
    fn can_build_robot(
        &self,
        robot_type: usize,
        blueprint: &Blueprint,
        max_materials: &[u32],
    ) -> bool {
        let recipe = &blueprint.robot_recipes[robot_type];
        let maxed_out = self.robots[robot_type] >= max_materials[robot_type];
        !maxed_out
            && recipe
                .iter()
                .all(|&(amount, material)| self.materials[material as usize] >= amount)
    }

    fn build_robot(&mut self, robot_type: usize, blueprint: &Blueprint) {
        self.robots[robot_type] += 1;
        for &(amount, material) in &blueprint.robot_recipes[robot_type] {
            self.materials[material as usize] -= amount;
        }
    }

    fn un_build_robot(&mut self, robot_type: usize, blueprint: &Blueprint) {
        self.robots[robot_type] -= 1;
        for &(amount, material) in &blueprint.robot_recipes[robot_type] {
            self.materials[material as usize] += amount;
        }
    }
}

type Input = Vec<Blueprint>;

fn get_blueprint_score(blueprint: &Blueprint, time_remaining: u32) -> u32 {
    let state = SearchState {
        time_remaining,
        robots: [1, 0, 0, 0],
        materials: [0, 0, 0, 0],
    };
    let max_materials = get_max_materials(blueprint);
    run_for_blueprint(&state, blueprint, &max_materials, None, 0)
}

fn run_for_blueprint(
    state: &SearchState,
    blueprint: &Blueprint,
    max_materials: &[u32],
    prev_skipped: Option<&Vec<usize>>,
    best_so_far: u32,
) -> u32 {
    if state.time_remaining == 1 {
        return state.materials[3] + state.robots[3];
    }

    if optimistic_best(state, Geode) < best_so_far {
        return 0;
    }

    let min_obsidian = max_materials[2];
    if optimistic_best(state, Obsidian) < min_obsidian {
        return state.materials[3] + state.robots[3] * state.time_remaining;
    }

    let mut new_state = *state;
    new_state.time_remaining -= 1;
    (0..4).for_each(|i| new_state.materials[i] += new_state.robots[i]);

    if state.can_build_robot(Geode as usize, blueprint, max_materials) {
        new_state.build_robot(Geode as usize, blueprint);
        return run_for_blueprint(&new_state, blueprint, max_materials, None, best_so_far);
    }

    let robots_available = (0..3)
        .filter(|i| state.can_build_robot(*i, blueprint, max_materials))
        .collect_vec();
    let mut best = best_so_far;

    for &robot_type in &robots_available {
        if prev_skipped
            .map(|ls| ls.contains(&robot_type))
            .unwrap_or(false)
        {
            continue;
        }

        new_state.build_robot(robot_type, blueprint);
        let score = run_for_blueprint(&new_state, blueprint, max_materials, None, best);
        best = max(score, best);
        new_state.un_build_robot(robot_type, blueprint);
    }

    let score = run_for_blueprint(
        &new_state,
        blueprint,
        max_materials,
        Some(&robots_available),
        best,
    );
    best = max(score, best);

    best
}

fn optimistic_best(state: &SearchState, material: Material) -> u32 {
    let mat = material as usize;
    let i = state.time_remaining;

    state.materials[mat] + state.robots[mat] * i + i * (i - 1) / 2
}

fn get_max_materials(blueprint: &Blueprint) -> [u32; 4] {
    let mut maxes = [0, 0, 0, u32::MAX];

    for recipe in &blueprint.robot_recipes {
        for &(amount, material) in recipe {
            let i = material as usize;
            maxes[i] = max(maxes[i], amount);
        }
    }
    maxes
}

fn parse(input: &str) -> Input {
    input.lines().filter_map(|line| {
        // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 7 obsidian.
        let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let c = regex.captures(line)?;
        let mut captures = c.iter().skip(1);

        let id = captures.next()??.as_str().parse().unwrap();

        let ore_robot_ore_cost = captures.next()??.as_str().parse().unwrap();
        let clay_robot_ore_cost = captures.next()??.as_str().parse().unwrap();
        let obsidian_robot_ore_cost = captures.next()??.as_str().parse().unwrap();
        let obsidian_robot_clay_cost = captures.next()??.as_str().parse().unwrap();
        let geode_robot_ore_cost = captures.next()??.as_str().parse().unwrap();
        let geode_robot_obsidian_cost = captures.next()??.as_str().parse().unwrap();

        let ore_robot: Vec<(u32, Material)> = vec![(ore_robot_ore_cost, Ore)];
        let clay_robot: Vec<(u32, Material)> = vec![(clay_robot_ore_cost, Ore)];
        let obsidian_robot: Vec<(u32, Material)> = vec![(obsidian_robot_ore_cost, Ore), (obsidian_robot_clay_cost, Clay)];
        let geode_robot: Vec<(u32, Material)> = vec![(geode_robot_ore_cost, Ore), (geode_robot_obsidian_cost, Obsidian)];

        Some(Blueprint {
            id,
            robot_recipes: [ore_robot, clay_robot, obsidian_robot, geode_robot],
        })
    }).collect_vec()
}

pub fn part_one(input: Input) -> Option<u32> {
    Some(
        input
            .par_iter()
            .map(|bp| (bp.id * get_blueprint_score(bp, 24)))
            .sum::<u32>(),
    )
}

pub fn part_two(input: Input) -> Option<u32> {
    Some(
        input
            .par_iter()
            .take(3)
            .map(|bp| get_blueprint_score(bp, 32))
            .product::<u32>(),
    )
}

advent_of_code::main!(19);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 19)));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 19)));
        assert_eq!(result, Some(3472));
    }
}

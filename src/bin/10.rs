use std::collections::{HashSet, VecDeque};

use z3::{Optimize, ast::Int};

const TEST: &str = include_str!("../../input/10/test.txt");
const INPUT: &str = include_str!("../../input/10/input.txt");

pub struct Machine {
    pub light_diag: Vec<bool>,
    pub button_schemas: Vec<Vec<usize>>,
    pub joltage_req: Vec<usize>,
}

fn solve_lights_req(goal: Vec<bool>, schemas: Vec<Vec<usize>>) -> usize {
    let mut seen: HashSet<Vec<bool>> = HashSet::new();
    let mut queue: VecDeque<(Vec<bool>, usize)> = VecDeque::new();

    queue.push_back((vec![false; goal.len()], 0));

    while let Some((state, iter)) = queue.pop_front() {
        if state == goal {
            return iter;
        } else if seen.contains(&state) {
            continue;
        }

        seen.insert(state.clone());

        schemas.iter().for_each(|button| {
            let mut next = state.clone();
            button.iter().for_each(|&index| next[index] = !next[index]);
            queue.push_back((next, iter + 1));
        })
    }

    panic!();
}

fn solve_joltage_req(goal: Vec<usize>, schemas: Vec<Vec<usize>>) -> usize {
    let opt = Optimize::new();

    let schemas: Vec<Vec<usize>> = schemas
        .iter()
        .map(|schema| {
            let mut delta: Vec<usize> = vec![0; goal.len()];
            schema.iter().for_each(|&index| delta[index] = 1);
            delta
        })
        .collect();

    let constants: Vec<Int> = (0..schemas.len())
        .map(|index| Int::fresh_const(format!("{index}").as_str()))
        .collect();

    for index in 0..goal.len() {
        let muls: Vec<Int> = constants
            .iter()
            .zip(schemas.iter())
            .map(|(constant, schema)| Int::mul(&[constant, &Int::from_u64(schema[index] as u64)]))
            .collect();

        opt.assert(&Int::add(&muls).eq(Int::from_u64(goal[index] as u64)))
    }

    constants.iter().for_each(|constant| {
        opt.assert(&constant.ge(Int::from_u64(0)));
    });

    let objective = Int::add(&constants);
    opt.minimize(&objective);

    assert!(opt.check(&[]) == z3::SatResult::Sat);
    let model = opt.get_model().unwrap();
    constants
        .iter()
        .map(|constant| model.get_const_interp(constant).unwrap().as_u64().unwrap())
        .sum::<u64>() as usize
}

fn part_1(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let diagram: Vec<bool> = parts[0][1..parts[0].len() - 1]
                .chars()
                .map(|light| match light {
                    '.' => false,
                    '#' => true,
                    other => unreachable!("parsed unknown char: {other}"),
                })
                .collect();
            let schematic = parts[1..parts.len() - 1]
                .iter()
                .map(|schema| {
                    schema[1..schema.len() - 1]
                        .split(',')
                        .map(|light| light.parse().unwrap())
                        .collect()
                })
                .collect();
            let requirement: Vec<usize> = parts[parts.len() - 1]
                [1..parts[parts.len() - 1].len() - 1]
                .split(',')
                .map(|joltage| joltage.parse().unwrap())
                .collect();

            Machine {
                light_diag: diagram,
                button_schemas: schematic,
                joltage_req: requirement,
            }
        })
        .collect();

    machines
        .iter()
        .map(|machine| solve_lights_req(machine.light_diag.clone(), machine.button_schemas.clone()))
        .sum()
}

fn part_2(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let diagram: Vec<bool> = parts[0][1..parts[0].len() - 1]
                .chars()
                .map(|light| match light {
                    '.' => false,
                    '#' => true,
                    other => unreachable!("parsed unknown char: {other}"),
                })
                .collect();
            let schematic = parts[1..parts.len() - 1]
                .iter()
                .map(|schema| {
                    schema[1..schema.len() - 1]
                        .split(',')
                        .map(|light| light.parse().unwrap())
                        .collect()
                })
                .collect();
            let requirement: Vec<usize> = parts[parts.len() - 1]
                [1..parts[parts.len() - 1].len() - 1]
                .split(',')
                .map(|joltage| joltage.parse().unwrap())
                .collect();

            Machine {
                light_diag: diagram,
                button_schemas: schematic,
                joltage_req: requirement,
            }
        })
        .collect();

    machines
        .iter()
        .map(|machine| {
            solve_joltage_req(machine.joltage_req.clone(), machine.button_schemas.clone())
        })
        .sum()
}

fn main() {
    println!("day 10");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

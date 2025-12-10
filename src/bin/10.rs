use memoize::memoize;
use z3::{Optimize, ast::Int};

const TEST: &str = include_str!("../../input/10/test.txt");
const INPUT: &str = include_str!("../../input/10/input.txt");

pub struct Machine {
    pub lights_req: Vec<bool>,
    pub joltage_req: Vec<usize>,
    pub buttons: Vec<Vec<usize>>,
}

#[memoize]
fn solve_lights_req(
    goal: Vec<bool>,
    current: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    iter: usize,
) -> usize {
    /* TODO: Make this BFS instead of DFS with memoization. */
    if goal == current || iter > 10 {
        return iter;
    }

    buttons
        .iter()
        .map(|button| {
            let mut next = current.clone();
            button.iter().for_each(|&index| next[index] = !next[index]);
            solve_lights_req(goal.clone(), next, buttons.clone(), iter + 1)
        })
        .min()
        .unwrap()
}

fn solve_joltage_req(goal: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    let opt = Optimize::new();

    let constants: Vec<(&[usize], Int)> = buttons
        .iter()
        .enumerate()
        .map(|(index, button)| {
            (
                button.as_slice(),
                Int::fresh_const(format!("{index}").as_str()),
            )
        })
        .collect();

    for index in 0..goal.len() {
        let muls: Vec<Int> = constants
            .iter()
            .map(|(button, constant)| Int::mul(&[constant, &Int::from_u64(button[index] as u64)]))
            .collect();

        opt.assert(&Int::add(&muls).eq(Int::from_u64(goal[index] as u64)))
    }

    for constant in constants.iter() {
        opt.assert(&constant.1.ge(Int::from_u64(0)));
    }

    let terms: Vec<&Int> = constants.iter().map(|(_, constant)| constant).collect();
    let objective = Int::add(&terms);
    opt.minimize(&objective);

    if opt.check(&[]) == z3::SatResult::Sat {
        let model = opt.get_model().unwrap();
        constants
            .iter()
            .map(|(_, constant)| model.get_const_interp(constant).unwrap().as_u64().unwrap())
            .sum::<u64>() as usize
    } else {
        panic!();
    }
}

fn part_1(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let lights_req: Vec<bool> = parts[0][1..parts[0].len() - 1]
                .chars()
                .map(|light| match light {
                    '.' => false,
                    '#' => true,
                    other => unreachable!("parsed unknown char: {other}"),
                })
                .collect();
            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|schematic| {
                    schematic[1..schematic.len() - 1]
                        .split(',')
                        .map(|light| light.parse().unwrap())
                        .collect()
                })
                .collect();
            Machine {
                lights_req,
                joltage_req: vec![],
                buttons,
            }
        })
        .collect();

    machines
        .iter()
        .map(|machine| {
            solve_lights_req(
                machine.lights_req.clone(),
                vec![false; machine.lights_req.len()],
                machine.buttons.clone(),
                0,
            )
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let joltage_req: Vec<usize> = parts[parts.len() - 1]
                [1..parts[parts.len() - 1].len() - 1]
                .split(',')
                .map(|req| req.parse().unwrap())
                .collect();
            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|schematic| {
                    let mut button: Vec<usize> = vec![0; joltage_req.len()];
                    schematic[1..schematic.len() - 1]
                        .split(',')
                        .for_each(|light| button[light.parse::<usize>().unwrap()] += 1);
                    button
                })
                .collect();
            Machine {
                lights_req: vec![],
                joltage_req,
                buttons,
            }
        })
        .collect();

    machines
        .iter()
        .map(|machine| solve_joltage_req(machine.joltage_req.clone(), machine.buttons.clone()))
        .sum()
}

fn main() {
    println!("day 10");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

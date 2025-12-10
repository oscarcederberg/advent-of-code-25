use memoize::memoize;

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
    if goal == current {
        return iter;
    } else if iter > 10 {
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

#[memoize]
fn solve_joltage_req(
    goal: Vec<usize>,
    current: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    iter: usize,
) -> usize {
    if goal == current {
        return iter;
    } else if current
        .iter()
        .enumerate()
        .any(|(index, &joltage)| joltage > goal[index])
    {
        return usize::MAX;
    }

    buttons
        .iter()
        .map(|button| {
            let mut next = current.clone();
            button.iter().for_each(|&index| next[index] += 1);
            solve_joltage_req(goal.clone(), next, buttons.clone(), iter + 1)
        })
        .min()
        .unwrap()
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
                    schematic[1..schematic.len() - 1]
                        .split(',')
                        .map(|light| light.parse().unwrap())
                        .collect()
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
        .map(|machine| {
            solve_joltage_req(
                machine.joltage_req.clone(),
                vec![0; machine.joltage_req.len()],
                machine.buttons.clone(),
                0,
            )
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

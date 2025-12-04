const TEST: &'static str = include_str!("../../input/03/test.txt");
const INPUT: &'static str = include_str!("../../input/03/input.txt");

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .map(|bank: Vec<u32>| {
            let (index, max_1) = bank
                .iter()
                .enumerate()
                .rev() // NOTE: max_by_key() returns last max, rev() to get first.
                .max_by_key(|&(_, value)| value)
                .unwrap();

            let value = if index == bank.len() - 1 {
                let max_2 = bank.iter().take(bank.len() - 1).max().unwrap();
                max_2 * 10 + max_1
            } else {
                let max_2 = bank.iter().skip(index + 1).max().unwrap();
                max_1 * 10 + max_2
            };

            value as usize
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .map(|bank: Vec<u32>| {
            let (mut skip, mut value) = (0, 0);
            for take in (bank.len() - 11)..=bank.len() {
                let (index, max) = bank
                    .iter()
                    .enumerate()
                    .take(take)
                    .skip(skip)
                    .rev() // NOTE: max_by_key() returns last max, rev() to get first.
                    .max_by_key(|&(_, value)| value)
                    .unwrap();
                (skip, value) = (index + 1, value * 10 + *max as usize);
            }

            value
        })
        .sum()
}

fn main() {
    println!("day 03");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

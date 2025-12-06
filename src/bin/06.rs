const TEST: &'static str = include_str!("../../input/06/test.txt");
const INPUT: &'static str = include_str!("../../input/06/input.txt");

pub enum Op {
    Add,
    Mul,
}

fn part_1(input: &str) -> usize {
    let number_rows = input.lines().count() - 1;
    let numbers: Vec<Vec<usize>> = input
        .lines()
        .take(number_rows)
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|op| match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            other => unreachable!("got unknown operator: {other}"),
        })
        .enumerate()
        .map(|(col, op)| {
            let init = if matches!(op, Op::Add) { 0 } else { 1 };
            numbers.iter().fold(init, |acc, row| {
                if matches!(op, Op::Add) {
                    acc + row[col]
                } else {
                    acc * row[col]
                }
            })
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let number_rows = input.lines().count() - 1;
    let numbers_per_row = input.lines().next().unwrap().split_whitespace().count();
    /* NOTE: The digits per number is not set. We need to calculate it. */
    let digits_per_number: Vec<usize> = input
        .lines()
        .last()
        .unwrap()
        .split(['+', '*'])
        /* NOTE: All line starts with '+' or '*', so split creates an initial empty element. */
        .skip(1)
        .enumerate()
        .map(|(index, whitespace)| {
            /* NOTE: Need to add one digit to the last number on each row. */
            whitespace.len() + if index == numbers_per_row - 1 { 1 } else { 0 }
        })
        .collect();

    let mut numbers: Vec<Vec<Option<usize>>> = (0..numbers_per_row)
        .map(|number| vec![None; digits_per_number[number]])
        .collect();

    let operators: Vec<Op> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|op| match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            other => unreachable!("got unknown operator: {other}"),
        })
        .collect();

    input.lines().take(number_rows).for_each(|line| {
        let mut current = 0;
        (0..numbers_per_row).for_each(|number_index| {
            let digits = digits_per_number[number_index];
            line[current..current + digits]
                .chars()
                .enumerate()
                .for_each(|(digit_index, char)| {
                    if let Some(digit) = char.to_digit(10) {
                        numbers[number_index][digit_index] =
                            if let Some(number) = numbers[number_index][digit_index] {
                                Some(number * 10 + digit as usize)
                            } else {
                                Some(digit as usize)
                            }
                    }
                });
            /* NOTE: We need to add one to skip the column separator. */
            current += digits + 1;
        })
    });

    numbers
        .iter()
        .enumerate()
        .map(|(row, number_row)| {
            let op = &operators[row];
            let init = if matches!(op, Op::Add) { 0 } else { 1 };
            number_row.iter().fold(init, |acc, number| {
                if matches!(op, Op::Add) {
                    acc + number.unwrap()
                } else {
                    acc * number.unwrap()
                }
            })
        })
        .sum()
}

fn main() {
    println!("day 06");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

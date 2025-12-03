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
        .fold(0, |sum, bank: Vec<u32>| {
            let (mut index, mut max_1) = (0, 0);
            for (i, battery) in bank.iter().enumerate() {
                if *battery > max_1 {
                    (index, max_1) = (i, *battery);
                }
            }

            let value = if index == bank.len() - 1 {
                let max_2 = bank[..bank.len() - 1].iter().max().unwrap();
                max_2 * 10 + max_1
            } else {
                let max_2 = bank.iter().skip(index + 1).max().unwrap();
                max_1 * 10 + *max_2
            };
            sum + value as usize
        })
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|char| char.to_digit(10).unwrap() as u32)
                .collect()
        })
        .fold(0, |sum, mut bank: Vec<u32>| sum + value)
}

fn main() {
    println!("day 03");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

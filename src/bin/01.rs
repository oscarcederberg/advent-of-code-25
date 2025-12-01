const TEST: &'static str = include_str!("../../input/01/test.txt");
const INPUT: &'static str = include_str!("../../input/01/input.txt");

pub enum Input {
    Left(i32),
    Right(i32),
}

fn part_1(input: &str) -> usize {
    use Input::*;

    let mut step: i32 = 50;
    let mut zeroes: usize = 0;
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            match (chars.next().unwrap()) {
                'L' => Left(chars.as_str().parse().unwrap()),
                'R' => Right(chars.as_str().parse().unwrap()),
                other => unreachable!("not L or R: {other}"),
            }
        })
        .for_each(|instr| {
            match (instr) {
                Left(steps) => step = (step - steps).rem_euclid(100),
                Right(steps) => step = (step + steps).rem_euclid(100),
            };
            if step == 0 {
                zeroes += 1;
            }
        });
    zeroes
}

fn part_2(input: &str) -> usize {
    use Input::*;

    let mut step: i32 = 50;
    let mut zeroes: usize = 0;
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            match (chars.next().unwrap()) {
                'L' => Left(chars.as_str().parse().unwrap()),
                'R' => Right(chars.as_str().parse().unwrap()),
                other => unreachable!("not L or R: {other}"),
            }
        })
        .for_each(|instr| {
            match (instr) {
                Left(steps) => {
                    for i in 0..steps {
                        step = (step - 1).rem_euclid(100);
                        if step == 0 {
                            zeroes += 1;
                        }
                    }
                }
                Right(steps) => {
                    for i in 0..steps {
                        step = (step + 1).rem_euclid(100);
                        if step == 0 {
                            zeroes += 1;
                        }
                    }
                }
            };
        });
    zeroes
}

fn main() {
    println!("day 01");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

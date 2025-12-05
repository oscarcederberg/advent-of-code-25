use std::collections::HashSet;

const TEST: &'static str = include_str!("../../input/05/test.txt");
const INPUT: &'static str = include_str!("../../input/05/input.txt");

fn part_1(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|range| {
            let (first, last) = range.split_once('-').unwrap();
            (
                first.parse::<usize>().unwrap(),
                last.parse::<usize>().unwrap(),
            )
        })
        .collect();

    ingredients
        .lines()
        .map(|id| id.parse::<usize>().unwrap())
        .filter(|id| ranges.iter().any(|(first, last)| id >= first && id <= last))
        .count()
}

fn part_2(input: &str) -> usize {
    let mut ranges: HashSet<(usize, usize)> = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|range| {
            let (first, last) = range.split_once('-').unwrap();
            (
                first.parse::<usize>().unwrap(),
                last.parse::<usize>().unwrap(),
            )
        })
        .collect();
    let mut fresh = 0;
    let mut current_id = 0;

    loop {
        let highest_range_containing_id = ranges
            .iter()
            .filter(|&(first, last)| current_id >= *first && current_id <= *last)
            .max_by_key(|&(_, last)| last);

        if let Some(&(first, last)) = highest_range_containing_id {
            /* NOTE: We need +1 to be inclusive. */
            fresh += last - current_id + 1;
            current_id = last + 1;
            ranges.remove(&(first, last));
        } else {
            let lowest_range_after_id = ranges
                .iter()
                .filter(|&(first, _)| current_id < *first)
                .min_by_key(|&(first, _)| first);

            if let Some(&(first, last)) = lowest_range_after_id {
                /* NOTE: We need +1 to be inclusive. */
                fresh += last - first + 1;
                current_id = last + 1;
                ranges.remove(&(first, last));
            } else {
                break;
            }
        }
    }
    fresh
}

fn main() {
    println!("day 05");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

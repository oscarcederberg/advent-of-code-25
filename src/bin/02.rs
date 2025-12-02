const TEST: &'static str = include_str!("../../input/02/test.txt");
const INPUT: &'static str = include_str!("../../input/02/input.txt");

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (first, last) = range.split_once('-').unwrap();
            first.parse::<usize>().unwrap()..=last.parse::<usize>().unwrap()
        })
        .filter(|id| {
            let string = id.to_string();
            let (left, right) = string.split_at(string.len() / 2);
            left == right
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (first, last) = range.split_once('-').unwrap();
            first.parse::<usize>().unwrap()..=last.parse::<usize>().unwrap()
        })
        .filter(|id| {
            let id = id.to_string();
            (1..id.len())
                .find(|chunk_len| {
                    if id.len() % *chunk_len != 0 {
                        false
                    } else {
                        let chunks = id.as_bytes().chunks(*chunk_len).collect::<Vec<&[u8]>>();
                        let first = chunks.first().unwrap();
                        chunks.iter().all(|x| x == first)
                    }
                })
                .is_some()
        })
        .sum()
}

fn main() {
    println!("day 02");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

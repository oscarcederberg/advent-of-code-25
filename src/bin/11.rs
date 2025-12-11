use std::collections::{HashMap, HashSet};

const TEST_1: &str = include_str!("../../input/11/test_1.txt");
const TEST_2: &str = include_str!("../../input/11/test_2.txt");
const INPUT: &str = include_str!("../../input/11/input.txt");

fn find_paths(current: &str, servers: &HashMap<&str, HashSet<&str>>) -> usize {
    if current == "out" {
        return 1;
    }

    let next = servers.get(current).unwrap();
    next.iter().map(|&server| find_paths(server, servers)).sum()
}

fn find_problematic_paths<'a>(
    seen: &mut HashMap<(&'a str, bool, bool), usize>,
    current: &'a str,
    servers: &HashMap<&str, HashSet<&'a str>>,
    mut dac_visited: bool,
    mut fft_visited: bool,
) -> usize {
    if current == "out" {
        if dac_visited && fft_visited {
            return 1;
        }
        return 0;
    } else if current == "dac" {
        assert!(!dac_visited);
        dac_visited = true;
    } else if current == "fft" {
        assert!(!fft_visited);
        fft_visited = true;
    }

    if !seen.contains_key(&(current, dac_visited, fft_visited)) {
        let next = servers.get(current).unwrap();
        let paths = next
            .iter()
            .map(|&server| find_problematic_paths(seen, server, servers, dac_visited, fft_visited))
            .sum();

        seen.insert((current, dac_visited, fft_visited), paths);
    }

    *seen.get(&(current, dac_visited, fft_visited)).unwrap()
}

fn part_1(input: &str) -> usize {
    let servers: HashMap<&str, HashSet<&str>> = input
        .lines()
        .map(|line| {
            let (head, tail) = line.split_once(':').unwrap();
            (head, tail.trim().split(' ').collect())
        })
        .collect();

    find_paths("you", &servers)
}

fn part_2(input: &str) -> usize {
    let servers: HashMap<&str, HashSet<&str>> = input
        .lines()
        .map(|line| {
            let (head, tail) = line.split_once(':').unwrap();
            (head, tail.trim().split(' ').collect())
        })
        .collect();

    find_problematic_paths(&mut HashMap::new(), "svr", &servers, false, false)
}

fn main() {
    println!("day 11");
    println!("part 1 (test): {}", part_1(TEST_1));
    println!("part 2 (test): {}", part_2(TEST_2));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

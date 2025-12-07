const TEST: &'static str = include_str!("../../input/07/test.txt");
const INPUT: &'static str = include_str!("../../input/07/input.txt");

#[derive(Clone)]
pub enum Cell {
    Empty,
    Splitter,
    Beam,
}

fn part_1(input: &str) -> usize {
    let mut grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|cell| match cell {
                    '.' => Cell::Empty,
                    '^' => Cell::Splitter,
                    'S' => Cell::Beam,
                    other => unreachable!("got unknown token: {other}"),
                })
                .collect()
        })
        .collect();
    let (height, width) = (grid.len(), grid[0].len());

    let mut splits = 0;
    for y in 0..(height - 1) {
        let beams: Vec<usize> = grid[y]
            .iter()
            .enumerate()
            .filter(|&(_, cell)| matches!(cell, Cell::Beam))
            .map(|tuple| tuple.0)
            .collect();

        for x in beams.into_iter() {
            if !matches!(grid[y + 1][x], Cell::Splitter) {
                grid[y + 1][x] = Cell::Beam;
                continue;
            }

            splits += 1;

            let (x_left, x_right) = (x as isize - 1, x + 1);

            if x_left >= 0 && matches!(grid[y + 1][x_left as usize], Cell::Empty) {
                grid[y + 1][x_left as usize] = Cell::Beam;
            }

            if x_right < width && matches!(grid[y + 1][x_right], Cell::Empty) {
                grid[y + 1][x_right] = Cell::Beam;
            }
        }
    }
    splits
}

fn part_2(input: &str) -> usize {
    let mut start_x = 0;
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|row| {
            row.chars()
                .enumerate()
                .map(|(x, cell)| match cell {
                    '.' => Cell::Empty,
                    '^' => Cell::Splitter,
                    'S' => {
                        start_x = x;
                        Cell::Empty
                    }
                    other => unreachable!("got unknown token: {other}"),
                })
                .collect()
        })
        .collect();
    let (height, width) = (grid.len(), grid[0].len());
    let mut next: Vec<usize> = vec![0; width];
    next[start_x] = 1;

    for y in 0..(height - 1) {
        let mut beams = vec![0; width];
        std::mem::swap(&mut beams, &mut next);

        for (x, timelines) in beams
            .iter()
            .enumerate()
            .filter(|&(_, timeline)| *timeline > 0)
        {
            if matches!(grid[y + 1][x], Cell::Empty) {
                next[x] += timelines;
                continue;
            }

            let (x_left, x_right) = (x as isize - 1, x + 1);

            if x_left >= 0 && matches!(grid[y + 1][x_left as usize], Cell::Empty) {
                next[x_left as usize] += timelines
            }

            if x_right < width && matches!(grid[y + 1][x_right], Cell::Empty) {
                next[x_right] += timelines
            }
        }
    }
    next.iter().sum()
}

fn main() {
    println!("day 07");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

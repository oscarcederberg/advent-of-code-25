const TEST: &str = include_str!("../../input/04/test.txt");
const INPUT: &str = include_str!("../../input/04/input.txt");

const ADJACENT_OFFSETS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '@' => true,
                    '.' => false,
                    other => unreachable!("got unexpected char: {other}"),
                })
                .collect()
        })
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &is_roll)| is_roll)
                .map(|(x, _)| {
                    ADJACENT_OFFSETS
                        .iter()
                        .map(|&(offset_x, offset_y)| {
                            let (neighbor_x, neighbor_y) =
                                (x as isize + offset_x, y as isize + offset_y);
                            if neighbor_x < 0
                                || neighbor_x >= cols as isize
                                || neighbor_y < 0
                                || neighbor_y >= rows as isize
                            {
                                false
                            } else {
                                grid[neighbor_y as usize][neighbor_x as usize]
                            }
                        })
                        .filter(|&x| x)
                        .count()
                })
                .filter(|&neighbours| neighbours < 4)
                .count()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '@' => true,
                    '.' => false,
                    other => unreachable!("got unexpected char: {other}"),
                })
                .collect()
        })
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut total_removed = 0;

    loop {
        let removed_rolls: Vec<(usize, usize)> = grid
            .clone()
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, is_roll)| *is_roll)
                    .map(|(x, _)| {
                        let neighbours = ADJACENT_OFFSETS
                            .iter()
                            .map(|&(offset_x, offset_y)| {
                                let (neighbor_x, neighbor_y) =
                                    (x as isize + offset_x, y as isize + offset_y);
                                if neighbor_x < 0
                                    || neighbor_x >= cols as isize
                                    || neighbor_y < 0
                                    || neighbor_y >= rows as isize
                                {
                                    false
                                } else {
                                    grid[neighbor_y as usize][neighbor_x as usize]
                                }
                            })
                            .filter(|&x| x)
                            .count();
                        (x, y, neighbours)
                    })
                    .filter(|&(_, _, neighbours)| neighbours < 4)
                    .map(|(x, y, _)| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        if removed_rolls.is_empty() {
            break;
        }

        removed_rolls.iter().for_each(|&(x, y)| grid[y][x] = false);
        total_removed += removed_rolls.len();
    }

    total_removed
}

fn main() {
    println!("day 04");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

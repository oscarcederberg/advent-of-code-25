const TEST: &'static str = include_str!("../../input/04/test.txt");
const INPUT: &'static str = include_str!("../../input/04/input.txt");

fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<(usize, usize, bool)>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '@' => (x, y, true),
                    '.' => (x, y, false),
                    other => unreachable!("got unexpected char: {other}"),
                })
                .collect()
        })
        .collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|&(_, _, is_roll)| *is_roll)
                .map(|&(x, y, _)| {
                    [
                        (0, -1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                        (0, 1),
                        (-1, 1),
                        (-1, 0),
                        (-1, -1),
                    ]
                    .iter()
                    .map(|&(offset_x, offset_y)| {
                        let (x, y): (isize, isize) = (x as isize + offset_x, y as isize + offset_y);
                        if x < 0 || x >= cols as isize || y < 0 || y >= rows as isize {
                            false
                        } else {
                            grid[y as usize][x as usize].2
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
    let mut grid: Vec<Vec<(usize, usize, bool)>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '@' => (x, y, true),
                    '.' => (x, y, false),
                    other => unreachable!("got unexpected char: {other}"),
                })
                .collect()
        })
        .collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut total_removed = 0;

    loop {
        let removed: Vec<(usize, usize)> = grid
            .clone()
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|&(_, _, is_roll)| *is_roll)
                    .map(|&(x, y, _)| {
                        (
                            x,
                            y,
                            [
                                (0, -1),
                                (1, -1),
                                (1, 0),
                                (1, 1),
                                (0, 1),
                                (-1, 1),
                                (-1, 0),
                                (-1, -1),
                            ]
                            .iter()
                            .map(|&(offset_x, offset_y)| {
                                let (x, y): (isize, isize) =
                                    (x as isize + offset_x, y as isize + offset_y);
                                if x < 0 || x >= cols as isize || y < 0 || y >= rows as isize {
                                    false
                                } else {
                                    grid[y as usize][x as usize].2
                                }
                            })
                            .filter(|&x| x)
                            .count(),
                        )
                    })
                    .filter(|&(_, _, neighbours)| neighbours < 4)
                    .map(|(x, y, _)| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        removed.iter().for_each(|&(x, y)| grid[y][x].2 = false);

        if removed.is_empty() {
            break;
        }

        total_removed += removed.len();
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

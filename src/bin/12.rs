const INPUT: &str = include_str!("../../input/12/input.txt");

const NUM_GIFTS: usize = 6;

fn part_1(input: &str) -> usize {
    let mut gift_shapes = [[[false; 3]; 3]; NUM_GIFTS];
    let mut christmas_trees: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    input.split("\n\n").enumerate().for_each(|(index, text)| {
        if index < NUM_GIFTS {
            text.lines().skip(1).enumerate().for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, char)| {
                    gift_shapes[index][y][x] = match char {
                        '.' => false,
                        '#' => true,
                        other => unreachable!("unknown char: {other}"),
                    }
                });
            })
        } else {
            text.lines().for_each(|tree| {
                let (size, gifts) = tree.split_once(": ").unwrap();
                let (width, length) = size.split_once("x").unwrap();
                christmas_trees.push((
                    width.parse().unwrap(),
                    length.parse().unwrap(),
                    gifts.split(' ').map(|num| num.parse().unwrap()).collect(),
                ));
            });
        }
    });

    christmas_trees
        .iter()
        .map(|(width, length, req)| (width / 3 * length / 3) >= req.iter().sum())
        .filter(|&fits| fits)
        .count()
}

fn main() {
    println!("day 12");
    println!("part 1 (test): 2");
    println!("part 1: {}", part_1(INPUT));
}

use geo::{Contains, Polygon, Rect, coord};
use itertools::Itertools;

const TEST: &str = include_str!("../../input/09/test.txt");
const INPUT: &str = include_str!("../../input/09/input.txt");

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        })
        .combinations(2)
        .fold(0, |max, points| {
            let ((x1, y1), (x2, y2)) = (points[0], points[1]);
            let width = isize::abs(x2 - x1) + 1;
            let height = isize::abs(y2 - y1) + 1;
            let area = (width * height) as usize;
            if max < area { area } else { max }
        })
}

fn part_2(input: &str) -> usize {
    let points: Vec<(f64, f64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap())
        })
        .collect();
    let polygon = Polygon::new(geo::LineString::from(points.clone()), vec![]);

    points.iter().combinations(2).fold(0, |max, points| {
        let (&(x1, y1), &(x2, y2)) = (points[0], points[1]);
        let rectangle = Rect::new(coord! { x: x1, y: y1}, coord! { x: x2, y: y2});

        if !polygon.contains(&rectangle) {
            return max;
        }

        let width = f64::abs(x2 - x1) + 1.0;
        let height = f64::abs(y2 - y1) + 1.0;
        let area = (width * height) as usize;
        if max < area { area } else { max }
    })
}

fn main() {
    println!("day 09");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

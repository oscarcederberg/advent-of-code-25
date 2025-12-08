use std::collections::HashSet;

use itertools::Itertools;

const TEST: &str = include_str!("../../input/08/test.txt");
const INPUT: &str = include_str!("../../input/08/input.txt");

pub type ID = usize;

#[derive(Debug)]
pub struct JunctionBox {
    pub id: ID,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

fn distance_squared(box_a: &JunctionBox, box_b: &JunctionBox) -> usize {
    let dx = box_a.x as isize - box_b.x as isize;
    let dy = box_a.y as isize - box_b.y as isize;
    let dz = box_a.z as isize - box_b.z as isize;
    (dx * dx + dy * dy + dz * dz) as usize
}

fn part_1(input: &str, connections: usize) -> usize {
    let boxes: Vec<JunctionBox> = input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let coords: Vec<usize> = line
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect();
            JunctionBox {
                id,
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();
    let shortest_distances: Vec<(ID, ID, usize)> = boxes
        .iter()
        .combinations(2)
        .map(|boxes| {
            (
                boxes[0].id,
                boxes[1].id,
                distance_squared(boxes[0], boxes[1]),
            )
        })
        .sorted_unstable_by_key(|&(_, _, distance_sq)| distance_sq)
        .collect();
    let mut circuits: Vec<HashSet<ID>> = boxes
        .iter()
        .map(|junction_box| HashSet::from([junction_box.id]))
        .collect();

    for &(a, b, _) in shortest_distances.iter().take(connections) {
        let circuit_a_i = circuits.iter().position(|circuit| circuit.contains(&a));
        let circuit_b_i = circuits.iter().position(|circuit| circuit.contains(&b));

        if let Some(circuit_a_i) = circuit_a_i
            && let Some(circuit_b_i) = circuit_b_i
            && circuit_a_i != circuit_b_i
        {
            let circuit_b = circuits[circuit_b_i].clone();
            circuits[circuit_a_i].extend(circuit_b);
            circuits.remove(circuit_b_i);
        }
    }

    circuits
        .iter()
        .sorted_unstable_by_key(|circuit| circuit.len())
        .rev()
        .take(3)
        .fold(1, |acc, circuit| acc * circuit.len())
}

fn part_2(input: &str) -> usize {
    let boxes: Vec<JunctionBox> = input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let coords: Vec<usize> = line
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect();
            JunctionBox {
                id,
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();
    let shortest_distances: Vec<(ID, ID, usize)> = boxes
        .iter()
        .combinations(2)
        .map(|boxes| {
            (
                boxes[0].id,
                boxes[1].id,
                distance_squared(boxes[0], boxes[1]),
            )
        })
        .sorted_unstable_by_key(|&(_, _, distance_sq)| distance_sq)
        .collect();
    let mut circuits: Vec<HashSet<ID>> = boxes
        .iter()
        .map(|junction_box| HashSet::from([junction_box.id]))
        .collect();
    let &(id_a, id_b, _) = shortest_distances
        .iter()
        .find(|&(a, b, _)| {
            let circuit_a_i = circuits.iter().position(|circuit| circuit.contains(a));
            let circuit_b_i = circuits.iter().position(|circuit| circuit.contains(b));

            if let Some(circuit_a_i) = circuit_a_i
                && let Some(circuit_b_i) = circuit_b_i
                && circuit_a_i != circuit_b_i
            {
                let circuit_b = circuits[circuit_b_i].clone();
                circuits[circuit_a_i].extend(circuit_b);
                circuits.remove(circuit_b_i);

                if circuits.len() == 1 {
                    return true;
                }
            }

            false
        })
        .unwrap();

    boxes[id_a].x * boxes[id_b].x
}

fn main() {
    println!("day 08");
    println!("part 1 (test): {}", part_1(TEST, 10));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT, 1000));
    println!("part 2: {}", part_2(INPUT));
}

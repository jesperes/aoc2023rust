use itertools::Itertools;
use nd_vec::vector;

use crate::Solver;
pub struct Solution;
impl Solver<i64, i64> for Solution {
    fn solve(&self, input: &str) -> (i64, i64) {
        (solve_p1(input), solve_p2(input))
    }
}

fn solve_p1(input: &str) -> i64 {
    let instructions: Vec<(char, _)> = input
        .lines()
        .map(|line| {
            let (dir, len) = line
                .split(|c| c == ' ' || c == '(' || c == '#' || c == ')')
                .filter(|s| !s.is_empty())
                .next_tuple()
                .unwrap();
            (dir.chars().next().unwrap(), len.parse().unwrap())
        })
        .collect();

    shoelace(instructions)
}

fn solve_p2(input: &str) -> i64 {
    let instructions: Vec<(char, _)> = input
        .lines()
        .map(|line| {
            let (_, hexstr) = line.split_once('#').unwrap();
            (
                match hexstr[5..6].chars().next().unwrap() {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => unreachable!(),
                },
                i64::from_str_radix(&hexstr[0..5], 16).unwrap(),
            )
        })
        .collect();

    shoelace(instructions)
}

fn shoelace(instructions: Vec<(char, i64)>) -> i64 {
    let mut pos = vector!(0, 0);
    let mut perimeter = 0;
    let mut area = 0;

    for (dir, steps) in instructions {
        let vector = match dir {
            'R' => vector!(1, 0),
            'D' => vector!(0, 1),
            'L' => vector!(-1, 0),
            'U' => vector!(0, -1),
            _ => unreachable!(),
        };

        let scaled_vector = vector * steps;
        pos += scaled_vector;
        perimeter += steps;
        area += pos.x() * scaled_vector.y();
    }

    area + perimeter / 2 + 1
}

use crate::Solver;
use hashbrown::HashSet;

use itertools::Itertools;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        let (p1, p2) = solve(input);
        (p1.to_string(), p2.to_string())
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    // Count how many "galaxies" there are on each row and column.
    let (rows, cols) = parse(input);

    let p1 = sum_of_galaxy_distances(input, &rows, &cols, 2);
    let p2 = sum_of_galaxy_distances(input, &rows, &cols, 1_000_000);

    (p1, p2)
}

fn parse(input: &str) -> (HashSet<usize>, HashSet<usize>) {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                rows.insert(row);
                cols.insert(col);
            }
        }
    }
    (rows, cols)
}

fn sum_of_galaxy_distances(
    input: &str,
    rows: &HashSet<usize>,
    cols: &HashSet<usize>,
    multiplier: usize,
) -> usize {
    let mut galaxies = HashSet::new();
    let mut row_offset = 0;

    for (row, line) in input.lines().enumerate() {
        let mut col_offset = 0;

        if !rows.contains(&row) {
            row_offset += multiplier - 1;
        }

        for (col, char) in line.chars().enumerate() {
            if !cols.contains(&col) {
                col_offset += multiplier - 1;
            }

            if char == '#' {
                galaxies.insert((row + row_offset, col + col_offset));
            }
        }
    }

    galaxies
        .iter()
        .cartesian_product(&galaxies)
        .filter(|(p1, p2)| p1 < p2)
        .map(|((row1, col1), (row2, col2))| row1.abs_diff(*row2) + col1.abs_diff(*col2))
        .sum()
}

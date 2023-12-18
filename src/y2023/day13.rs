pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &str) -> (String, String) {
        let (p1, p2) = solve(input);
        (p1.to_string(), p2.to_string())
    }
}

use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

use crate::Solver;

// Represent the mirror as a vector of ints
type MirrorBits = Vec<u32>;

// Detect reflections with exactly 'smudges' number of incorrect bits.
fn mirrors(ns: &MirrorBits, i: usize, smudges: u32) -> bool {
    (0..i)
        .rev()
        .zip(i..ns.len())
        .map(|(a, b)| (ns[a] ^ ns[b]).count_ones())
        .sum::<u32>()
        == smudges
}

fn summarize(grid: &str, smudges: u32) -> usize {
    let mut rows = MirrorBits::new();
    let mut cols = MirrorBits::new();

    for line in grid.lines() {
        cols.resize(line.len(), 0);
        let mut row = 0;
        for (c, v) in line.bytes().enumerate() {
            cols[c] = (cols[c] << 1) | ((v == b'#') as u32);
            row = (row << 1) | ((v == b'#') as u32);
        }
        rows.push(row);
    }

    (1..cols.len())
        // has vertical reflection?
        .find(|c| mirrors(&cols, *c, smudges))
        .or_else(|| {
            // if not, check horizontal reflection and multiply by 100
            Some(
                (1..rows.len())
                    .find(|r| mirrors(&rows, *r, smudges))
                    .unwrap_or(0)
                    * 100,
            )
        })
        .unwrap()
}

pub fn solve(input: &str) -> (usize, usize) {
    // Solve p1 and p2 in parallel
    let mut solutions = vec![0, 1]
        .par_iter()
        .map(|part| {
            let sort_key = *part as usize;
            let smudge = *part;
            let solution = do_solve(input, smudge);
            (sort_key, solution)
        })
        .collect::<Vec<(usize, usize)>>();

    solutions.sort();
    (solutions[0].1, solutions[1].1)
}

fn do_solve(input: &str, smudges: u32) -> usize {
    input
        .split("\n\n")
        .par_bridge()
        .map(|grid| summarize(grid, smudges))
        .sum()
}

use itertools::Itertools;

use crate::Solver;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        let input = input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        (
            do_solve(&input, &predict_next).to_string(),
            do_solve(&input, &predict_prev).to_string(),
        )
    }
}

fn do_solve(input: &Vec<Vec<i64>>, next_fun: &dyn Fn(&Vec<i64>) -> i64) -> i64 {
    input.iter().map(next_fun).sum()
}

fn predict_next(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|n| *n == 0) {
        0
    } else {
        seq.last().unwrap() + predict_next(&next_seq(seq))
    }
}

fn predict_prev(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|n| *n == 0) {
        0
    } else {
        seq.first().unwrap() - predict_prev(&next_seq(seq))
    }
}

fn next_seq(sequence: &Vec<i64>) -> Vec<i64> {
    sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

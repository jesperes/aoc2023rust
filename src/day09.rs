extern crate test;

use itertools::Itertools;

pub fn solve() -> (i64, i64) {
    let input = include_str!("../inputs/input09.txt")
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    (
        do_solve(&input, &predict_next),
        do_solve(&input, &predict_prev),
    )
}

fn do_solve(input: &Vec<Vec<i64>>, next_fun: &dyn Fn(&Vec<i64>) -> i64) -> i64 {
    input.iter().map(|seq| next_fun(&seq)).sum()
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        b.iter(|| assert_eq!((1972648895, 919), solve()))
    }
}

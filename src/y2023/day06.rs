use itertools::Itertools;

use crate::Solver;
pub struct Solution;
impl Solver<i64, i64> for Solution {
    fn solve(&self, input: &str) -> (i64, i64) {
        solve(input)
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    let (times, dists) = input.split_once('\n').unwrap();
    let p1 = solve_p1(times, dists);
    let p2 = solve_p2(times, dists);
    (p1, p2)
}

fn solve_p1(times: &str, dists: &str) -> i64 {
    let time_nums = get_nums(times);
    let dist_nums = get_nums(dists);
    time_nums
        .iter()
        .zip(dist_nums.iter())
        .fold(1, |acc, (time, record)| acc * find_holdtime(*time, *record))
}

fn solve_p2(times: &str, dists: &str) -> i64 {
    let time = get_num(times);
    let dist = get_num(dists);
    find_holdtime(time, dist)
}

fn get_num(line: &str) -> i64 {
    line[12..]
        .chars()
        .filter(|s| s.is_numeric())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}
fn get_nums(line: &str) -> Vec<i64> {
    line[12..]
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec()
}

fn find_holdtime(time: i64, record: i64) -> i64 {
    let b = -time as f64;
    let c = record as f64;
    let sq = (b * b - 4f64 * c).sqrt();
    let x0 = (-b - sq) / 2f64;
    let x1 = (-b + sq) / 2f64;
    (x1.floor() - x0.floor()) as i64
}

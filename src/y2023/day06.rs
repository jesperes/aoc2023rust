use crate::Solver;
pub struct Solution;
impl Solver<u64, u64> for Solution {
    fn solve(&self, input: &str) -> (u64, u64) {
        let mut lines = input.lines();
        let times = lines.next().unwrap();
        let dists = lines.next().unwrap();
        (solve_p1(times, dists), solve_p2(times, dists))
    }
}

fn solve_p1(times: &str, dists: &str) -> u64 {
    times
        .split_whitespace()
        .zip(dists.split_whitespace())
        .skip(1)
        .map(|(t, d)| (t.parse().unwrap(), d.parse().unwrap()))
        .fold(1, |acc, (time, dist)| acc * find_holdtime(time, dist))
}

fn solve_p2(times: &str, dists: &str) -> u64 {
    let time = parse_number_with_spaces(&times[10..]);
    let dist = parse_number_with_spaces(&dists[10..]);
    find_holdtime(time, dist)
}

fn parse_number_with_spaces(input: &str) -> u64 {
    input
        .bytes()
        .filter(|&c| c != b' ')
        .fold(0, |acc, x| acc * 10 + (x - b'0') as u64)
}

fn find_holdtime(time: u64, distance: u64) -> u64 {
    let d = ((time * time - 4 * distance) as f64).sqrt();
    let t = time as f64;
    let x0 = ((t - d) / 2.0).ceil() as u64;
    let x1 = ((t + d) / 2.0).floor() as u64;
    x1 - x0 + 1
}

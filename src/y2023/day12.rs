use hashbrown::HashMap;
use itertools::Itertools;

use crate::Solver;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        let (p1, p2) = solve(input);
        (p1.to_string(), p2.to_string())
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    let p1 = sum_arrangements(input, 1);
    let p2 = sum_arrangements(input, 5);
    (p1, p2)
}

trait Duplicatable {
    fn duplicate_with_sep(&self, copies: i32, sep: char) -> String;
}

impl Duplicatable for String {
    fn duplicate_with_sep(&self, copies: i32, sep: char) -> String {
        if copies == 1 {
            return self.clone();
        }

        let mut copy = self.clone();
        for _i in 0..(copies - 1) {
            copy.push(sep);
            copy.push_str(self.as_str());
        }
        copy
    }
}

fn sum_arrangements(input: &str, copies: i32) -> i64 {
    input
        .lines()
        .map(|line| count_arrangements(line, copies))
        .sum()
}

fn count_arrangements(line: &str, copies: i32) -> i64 {
    let mut cache = HashMap::new();
    let (left, right) = line.split_once(' ').unwrap();
    let counts = right
        .to_string()
        .duplicate_with_sep(copies, ',')
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec();

    let record = format!("{}.", left.to_string().duplicate_with_sep(copies, '?'));

    get_count(record.as_bytes(), &counts, 0, 0, 0, &mut cache)
}

fn get_count(
    line: &[u8],
    counts: &Vec<i64>,
    pos: usize,
    curr: i64,
    count_pos: usize,
    cache: &mut HashMap<(usize, i64, usize), i64>,
) -> i64 {
    let key = (pos, curr, count_pos);

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let result = if pos == line.len() {
        if counts.len() == count_pos {
            1
        } else {
            0
        }
    } else if line[pos] == b'#' {
        get_count(line, counts, pos + 1, curr + 1, count_pos, cache)
    } else if line[pos] == b'.' || count_pos == counts.len() {
        if count_pos < counts.len() && curr == counts[count_pos] {
            get_count(line, counts, pos + 1, 0, count_pos + 1, cache)
        } else if curr == 0 {
            get_count(line, counts, pos + 1, 0, count_pos, cache)
        } else {
            0
        }
    } else {
        let hash_count = get_count(line, counts, pos + 1, curr + 1, count_pos, cache);
        let dot_count = if curr == counts[count_pos] {
            get_count(line, counts, pos + 1, 0, count_pos + 1, cache)
        } else if curr == 0 {
            get_count(line, counts, pos + 1, 0, count_pos, cache)
        } else {
            0
        };
        hash_count + dot_count
    };

    cache.insert(key, result);
    result
}

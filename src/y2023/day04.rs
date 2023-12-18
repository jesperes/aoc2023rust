use hashbrown::{HashMap, HashSet};

use crate::Solver;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &str) -> (String, String) {
        solve(input)
    }
}

pub fn solve(input: &str) -> (String, String) {
    let mut map: HashMap<i32, i32> = HashMap::new();

    let (num_cards, sum) = input.lines().filter_map(|line| line.split_once(':')).fold(
        (0, 0),
        |(n, sum), (left, right)| {
            let card_num = left[4..].trim().parse::<i32>().unwrap();
            let (s1, s2) = right.split_once('|').unwrap();
            let num_matching = split_nums(s1).intersection(&split_nums(s2)).count() as i32;

            for i in (card_num + 1)..=(card_num + num_matching) {
                *map.entry(i).or_insert(0) += map.get(&card_num).unwrap_or(&0) + 1;
            }

            let sum0 = sum
                + if num_matching <= 0 {
                    0
                } else {
                    1 << (num_matching - 1)
                };

            (n + 1, sum0)
        },
    );

    (
        sum.to_string(),
        (map.values().sum::<i32>() + num_cards).to_string(),
    )
}

fn split_nums(s: &str) -> HashSet<i32> {
    s.split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

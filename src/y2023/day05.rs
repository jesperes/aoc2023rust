trace::init_depth_var!();

use itertools::Itertools;

use crate::Solver;

pub struct Solution;
impl Solver<i64, i64> for Solution {
    fn solve(&self, input: &str) -> (i64, i64) {
        solve(input)
    }
}

fn solve(input: &str) -> (i64, i64) {
    let p1 = solve_p1(input);
    // let p2 = solve_p2(input);
    // (p1, p2)
    (p1, 0)
}

fn solve_p1(input: &str) -> i64 {
    let sections = input.split("\n\n").collect_vec();
    let seeds = sections[0]
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    let steps = sections[1..]
        .iter()
        .map(|sec| {
            sec.split('\n')
                .skip(1)
                .map(|line| {
                    line.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_vec()
                })
                .filter(|s| !s.is_empty())
                .collect_vec()
        })
        .collect_vec();

    let mut ans = i64::MAX;

    for mut s in seeds {
        for step in &steps {
            for range in step {
                if let [dst, src, sz] = range[..] {
                    let src_end = src + sz - 1;
                    if src <= s && s <= src_end {
                        s = s - src + dst;
                        break;
                    }
                } else {
                    unreachable!()
                }
            }
        }

        ans = ans.min(s)
    }

    ans
}

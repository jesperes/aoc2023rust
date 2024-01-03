trace::init_depth_var!();

use std::collections::VecDeque;

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
    let p2 = solve_p2(input);
    (p1, p2)
}

fn solve_p1(input: &str) -> i64 {
    let sections = input.split("\n\n").collect_vec();
    let mut seeds = sections[0]
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

    *seeds
        .iter_mut()
        .map(|s| {
            for step in &steps {
                for range in step {
                    if let [dst, src, sz] = range[..] {
                        let src_end = src + sz - 1;
                        if src <= *s && *s <= src_end {
                            *s = *s - src + dst;
                            break;
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            s
        })
        .min()
        .unwrap()
}

fn overlap(a: i64, b: i64, c: i64, d: i64) -> bool {
    !(a > d || b < c)
}

fn solve_p2(input: &str) -> i64 {
    let sections = input.split("\n\n").collect_vec();
    let mut cur = sections[0]
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .tuples::<(_, _)>()
        .map(|(a, b)| (a, a + b - 1))
        .collect::<VecDeque<_>>();

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

    for step in &steps {
        let mut new = VecDeque::new();

        while let Some((a, b)) = cur.pop_front() {
            let mut found_rule = false;

            for range in step {
                if let [dst, src, sz] = range[..] {
                    let (c, d) = (src, src + sz - 1);
                    let delta = dst - src;

                    if !overlap(a, b, c, d) {
                        continue;
                    }

                    if c <= a && a <= d && c <= b && b <= d {
                        new.push_back((a + delta, b + delta));
                        found_rule = true;
                        break;
                    } else if c <= b && b <= d {
                        new.push_back((c + delta, b + delta));
                        cur.push_back((a, c - 1));
                        found_rule = true;
                        break;
                    } else if c <= a && a <= d {
                        new.push_back((a + delta, d + delta));
                        cur.push_back((d + 1, b));
                        found_rule = true;
                        break;
                    } else if a < c && b > d {
                        new.push_back((c + delta, d + delta));
                        cur.push_back((d + 1, b));
                        cur.push_back((a, c - 1));
                        found_rule = true;
                        break;
                    }
                }
            }

            if !found_rule {
                new.push_back((a, b));
            }
        }

        cur = new;
    }

    *cur.iter().map(|(a, _)| a).min().unwrap()
}

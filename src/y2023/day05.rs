trace::init_depth_var!();

use std::collections::VecDeque;

use itertools::Itertools;

use crate::Solver;

type Steps = Vec<Vec<(i64, i64, i64)>>;

pub struct Solution;
impl Solver<i64, i64> for Solution {
    fn solve(&self, input: &str) -> (i64, i64) {
        solve(input)
    }
}

fn solve(input: &str) -> (i64, i64) {
    let (header, rest) = input.split_once("\n\n").unwrap();
    let sections = parse_sections(rest);
    let p1 = solve_p1(header, &sections);
    let p2 = solve_p2(header, &sections);
    (p1, p2)
}

fn solve_p1(header: &str, steps: &Steps) -> i64 {
    let mut seeds = parse_header(header);
    *seeds
        .iter_mut()
        .map(|s| {
            for step in steps {
                for (dst, src, sz) in step {
                    let src_end = src + sz - 1;
                    if src <= s && *s <= src_end {
                        *s = *s - src + dst;
                        break;
                    }
                }
            }
            s
        })
        .min()
        .unwrap()
}

fn solve_p2(header: &str, steps: &Steps) -> i64 {
    let mut cur: VecDeque<(i64, i64)> = parse_header(header)
        .iter()
        .tuples::<(_, _)>()
        .map(|(a, b)| (*a, a + b - 1))
        .collect::<VecDeque<_>>();

    for step in steps {
        let mut new = VecDeque::new();

        while let Some((a, b)) = cur.pop_front() {
            let mut found_rule = false;

            for (dst, src, sz) in step {
                let (c, d) = (*src, src + sz - 1);
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

            if !found_rule {
                new.push_back((a, b));
            }
        }

        cur = new;
    }

    *cur.iter().map(|(a, _)| a).min().unwrap()
}

fn parse_header(header: &str) -> Vec<i64> {
    header
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec()
}

fn parse_sections(sections: &str) -> Steps {
    sections
        .split("\n\n")
        .map(|sec| {
            sec.split('\n')
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|line| {
                    line.split(' ')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_tuple::<(_, _, _)>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec()
}

fn overlap(a: i64, b: i64, c: i64, d: i64) -> bool {
    !(a > d || b < c)
}

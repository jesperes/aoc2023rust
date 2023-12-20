use hashbrown::HashMap;
use lazy_regex::regex_captures;

#[derive(Debug)]
enum Rule<'a> {
    Lt(&'a str, usize, &'a str),
    Gt(&'a str, usize, &'a str),
    Default(&'a str),
}

use crate::Solver;
pub struct Solution;
impl Solver<usize, usize> for Solution {
    fn solve(&self, input: &str) -> (usize, usize) {
        solve(input)
    }
}

type Range = (usize, usize);
type Ranges = (Range, Range, Range, Range);

fn solve(input: &str) -> (usize, usize) {
    let (section1, section2) = input.split_once("\n\n").unwrap();

    let workflows = section1
        .lines()
        .map(|line| {
            let (_, name, rules) = regex_captures!(r"(\w+)\{(.*)\}", line).unwrap();
            let rules = rules
                .split(',')
                .map(|rule| {
                    if let Some((_, cat, op, val, dest)) =
                        regex_captures!(r"(\w)(.)(\d+):(\w+)", rule)
                    {
                        if op == "<" {
                            Rule::Lt(cat, val.parse().unwrap(), dest)
                        } else if op == ">" {
                            Rule::Gt(cat, val.parse().unwrap(), dest)
                        } else {
                            unreachable!()
                        }
                    } else {
                        Rule::Default(rule)
                    }
                })
                .collect::<Vec<_>>();

            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    let parts = section2
        .lines()
        .map(|line| {
            let (_, x, m, a, s) =
                regex_captures!(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}", line).unwrap();
            (
                x.parse::<usize>().unwrap(),
                m.parse::<usize>().unwrap(),
                a.parse::<usize>().unwrap(),
                s.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let p1 = parts
        .iter()
        .filter_map(|part| process_workflow("in", part, &workflows))
        .sum();

    let ranges = ((1, 4000), (1, 4000), (1, 4000), (1, 4000));

    let p2 = process_workflow2("in", ranges, &workflows);

    (p1, p2)
}

fn process_workflow(
    name: &str,
    values: &(usize, usize, usize, usize),
    workflows: &HashMap<&str, Vec<Rule>>,
) -> Option<usize> {
    let rules = workflows.get(name).unwrap();
    let (x, m, a, s) = values;

    // Apply first matching rule
    let next_workflow = rules
        .iter()
        .find_map(|rule| match rule {
            Rule::Lt("x", val, dest) if x < val => Some(dest),
            Rule::Lt("m", val, dest) if m < val => Some(dest),
            Rule::Lt("a", val, dest) if a < val => Some(dest),
            Rule::Lt("s", val, dest) if s < val => Some(dest),
            Rule::Gt("x", val, dest) if x > val => Some(dest),
            Rule::Gt("m", val, dest) if m > val => Some(dest),
            Rule::Gt("a", val, dest) if a > val => Some(dest),
            Rule::Gt("s", val, dest) if s > val => Some(dest),
            Rule::Default(dest) => Some(dest),
            _ => None,
        })
        .unwrap();

    if *next_workflow == "A" {
        Some(x + m + a + s)
    } else if *next_workflow == "R" {
        None
    } else {
        process_workflow(next_workflow, values, workflows)
    }
}

fn get_category_range(cat: &str, ranges: Ranges) -> Range {
    let (x, m, a, s) = ranges;
    match cat {
        "x" => x,
        "m" => m,
        "a" => a,
        "s" => s,
        _ => unreachable!(),
    }
}

fn set_cat_range(cat: &str, ranges: Ranges, new_range: Range) -> Ranges {
    let (x, m, a, s) = ranges;
    match cat {
        "x" => (new_range, m, a, s),
        "m" => (x, new_range, a, s),
        "a" => (x, m, new_range, s),
        "s" => (x, m, a, new_range),
        _ => unreachable!(),
    }
}

fn process_workflow2(name: &str, ranges: Ranges, workflows: &HashMap<&str, Vec<Rule>>) -> usize {
    if name == "A" {
        let ((x0, x1), (m0, m1), (a0, a1), (s0, s1)) = ranges;
        let x = x1 - x0 + 1;
        let m = m1 - m0 + 1;
        let a = a1 - a0 + 1;
        let s = s1 - s0 + 1;
        return x * m * a * s;
    } else if name == "R" {
        return 0;
    }

    workflows
        .get(name)
        .unwrap()
        .iter()
        .fold((0, ranges), |(n, ranges), rule| match rule {
            Rule::Lt(cat, val, dest) => {
                let (min, max) = get_category_range(cat, ranges);
                if *val <= min {
                    (n, ranges)
                } else {
                    let true_ranges = set_cat_range(cat, ranges, (min, val - 1));
                    let false_ranges = set_cat_range(cat, ranges, (*val, max));
                    let count = process_workflow2(dest, true_ranges, workflows);
                    (n + count, false_ranges)
                }
            }
            Rule::Gt(cat, val, dest) => {
                let (min, max) = get_category_range(cat, ranges);
                if *val >= max {
                    (n, ranges)
                } else {
                    let true_ranges = set_cat_range(cat, ranges, (val + 1, max));
                    let false_ranges = set_cat_range(cat, ranges, (min, *val));
                    let count = process_workflow2(dest, true_ranges, workflows);
                    (n + count, false_ranges)
                }
            }
            Rule::Default(dest) => (n + process_workflow2(dest, ranges, workflows), ranges),
        })
        .0
}

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

fn set_category_range(cat: &str, ranges: Ranges, new_range: Range) -> Ranges {
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
        println!(
            "Reached accept state with ranges {:?} -> {:?} == {:?}",
            ranges,
            (x, m, a, s),
            x * m * a * s
        );
        return x * m * a * s;
    } else if name == "R" {
        return 0;
    }

    let wf = workflows.get(name).unwrap();
    wf.iter()
        .fold((0, ranges), |(n, ranges), rule| {
            println!("Processing rule {:?}", rule);
            match rule {
                Rule::Lt(cat, val, dest) => {
                    let (min, max) = get_category_range(cat, ranges);
                    if *val <= min {
                        // This rule can not apply for any ranges of values for
                        // x, m, a, and s.
                        (n, ranges)
                    } else {
                        // For the range of number making this condition true,
                        // recurse down and investigate other workflows
                        let true_ranges = set_category_range(cat, ranges, (min, val - 1));
                        let count = process_workflow2(dest, true_ranges, workflows);

                        // The resulting range for this condition being false is passed to the
                        // next iteration in the loop.
                        let false_ranges = set_category_range(cat, ranges, (*val, max));

                        (n + count, false_ranges)
                    }
                }
                Rule::Gt(cat, val, dest) => {
                    let (min, max) = get_category_range(cat, ranges);
                    if *val >= max {
                        // This rule can not apply for any ranges of values for
                        // x, m, a, and s.
                        (n, ranges)
                    } else {
                        // For the range of number making this condition true,
                        // recurse down and investigate other workflows
                        let true_ranges = set_category_range(cat, ranges, (val + 1, max));
                        let count = process_workflow2(dest, true_ranges, workflows);

                        // The resulting range for this condition being false is passed to the
                        // next iteration in the loop.
                        let false_ranges = set_category_range(cat, ranges, (min, *val));

                        (n + count, false_ranges)
                    }
                }
                Rule::Default(dest) => (n + process_workflow2(dest, ranges, workflows), ranges),
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        let ex = "\
in{x<1000:ab,m<1000:A,R}
ab{m<500:A,R}

{x=0,m=0,a=0,s=0}
";
        // two accept states:
        // A1: x<1000, m<500 => 999 * 499 * 4000 * 4000 = 7976016000000
        // A2: x>=1000, m<1000 => 3001 * 999 * 4000 * 4000 = 47967984000000
        let (_, p2) = solve(ex);
        assert_eq!(7976016000000 + 47967984000000, p2);
    }
    #[test]
    fn test_name() {
        let ex = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!((19114, 167409079868000), solve(ex));
    }
}

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
impl Solver<usize, i32> for Solution {
    fn solve(&self, input: &str) -> (usize, i32) {
        solve(input)
    }
}

fn solve(input: &str) -> (usize, i32) {
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

    (p1, 0)
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
#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!((19114, 0), solve(ex));
    }
}

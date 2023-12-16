pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        let (p1, p2) = solve(input);
        (p1.to_string(), p2.to_string())
    }
}

use hashbrown::HashMap;

use crate::Solver;

pub fn solve(input: &str) -> (i32, usize) {
    let p1 = solve_p1(input);
    let p2 = solve_p2(input);
    (p1, p2)
}

fn solve_p1(input: &str) -> i32 {
    input.split(',').map(hash).sum()
}

fn solve_p2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = Vec::new();
    let mut lenses: HashMap<String, (usize, usize)> = HashMap::new();
    boxes.resize_with(256, || Vec::new());

    for s in input.split(',').map(|s| s.trim()) {
        let (label, cmd) = s.split_once(|c| c == '=' || c == '-').unwrap();
        let h = hash(label) as usize;

        if cmd == "" {
            // remove lens
            boxes[h].retain(|(l, _)| l != label);
            lenses.remove(label);
        } else {
            // add lens
            let focal_length = cmd.parse().unwrap();
            match boxes[h].iter_mut().find(|(lbl, _fl)| lbl == label) {
                Some((_, fl)) => {
                    *fl = focal_length;
                    lenses
                        .entry(label.to_string())
                        .or_insert_with(|| (h, focal_length))
                        .1 = focal_length;
                }
                None => {
                    boxes[h].push((label.to_string(), focal_length));
                    lenses.insert(label.to_string(), (h, focal_length));
                }
            }
        }
    }

    lenses
        .iter()
        .map(|(lens, (i, fl))| {
            (i + 1) * (boxes[*i].iter().position(|(l, _)| l == lens).unwrap() + 1) * fl
        })
        .sum()
}

fn hash(s: &str) -> i32 {
    s.trim()
        .chars()
        .fold(0, |acc, c| ((acc + c as i32) * 17) & 0xff)
}

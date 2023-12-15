use hashbrown::{HashMap, HashSet};
use itertools::{self, Itertools};

use crate::Solver;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        solve(input)
    }
}

fn is_digit(c: &u8) -> bool {
    *c >= b'0' && *c <= b'9'
}

pub fn solve(input: &str) -> (String, String) {
    let bytes = input.as_bytes();
    let w: i32 = bytes.iter().position(|&c| c == b'\n').unwrap() as i32 + 1; // include newline

    let mut map: HashMap<(usize, char), HashSet<(usize, i32)>> = HashMap::new();
    let mut numbers: Vec<(usize, usize, i32)> = Vec::new();

    let mut is_num = false;
    let mut curr_num = 0;
    let mut num_len = 0;
    let mut num_start = 0;

    // Look, ma! No regexps!
    for (i, b) in bytes.iter().enumerate() {
        match b {
            d if is_digit(d) => {
                if is_num {
                    curr_num = curr_num * 10 + (d - b'0') as i32;
                    num_len += 1;
                } else {
                    is_num = true;
                    num_start = i;
                    curr_num = (d - b'0') as i32;
                    num_len = 1;
                }
            }
            nd if !is_digit(nd) => {
                if is_num {
                    numbers.push((num_start, num_len, curr_num));
                    curr_num = 0;
                    is_num = false;
                    num_len = 0;
                }
            }
            _ => unreachable!(),
        }
    }

    for (start, len, num) in &numbers {
        // loop through the digits
        for i_unsigned in *start..(start + len) {
            let i = i_unsigned as i32;
            for adjidx in vec![
                i - 1,
                i + 1,
                i - w - 1,
                i - w,
                i - w + 1,
                i + w - 1,
                i + w,
                i + w + 1,
            ] {
                if adjidx >= 0 && adjidx < bytes.len() as i32 {
                    let c = bytes[adjidx as usize];
                    if is_digit(&c) || c == b'\n' || c == b'.' {
                        continue;
                    } else {
                        map.entry((adjidx as usize, c as char))
                            .or_insert_with(|| HashSet::new())
                            .insert((*start, *num));
                    }
                }
            }
        }
    }

    // For p1, sum all the values found in the map
    let p1 = map.values().fold(0, |acc, set| {
        set.iter().map(|(_, num)| num).sum::<i32>() + acc
    });

    // For p2, sum all the products of pairs of *-adjacent numbers.
    let p2 = map
        .iter()
        .filter(|((_, c), _)| *c == '*')
        .fold(0, |acc, (_key, set)| {
            if set.len() == 2 {
                let ((_, gear1), (_, gear2)) = set.iter().next_tuple().unwrap();
                acc + gear1 * gear2
            } else {
                acc
            }
        });

    (p1.to_string(), p2.to_string())
}

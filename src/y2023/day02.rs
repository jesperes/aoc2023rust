use itertools::Itertools;
use lazy_regex::regex;

use crate::Solver;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        solve(input)
    }
}

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

pub fn solve(input: &str) -> (String, String) {
    // A bit for fun, solve both p1 and p2 in a single statement
    let (p1, p2) = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .fold((0, 0), |(p1, p2), line| {
            let (s1, s2) = line.split(':').next_tuple().unwrap();
            let game = s1[5..].parse::<i32>().unwrap();

            // Note that we do not need to care about the different "sets" drawn
            // from the bag; we can just look at each draw of colored balls on
            // its own.
            let (game_id, (r, g, b)) = regex!(r"\d+ [rgb]").find_iter(s2).fold(
                (Some(game), (0, 0, 0)),
                |(game_id, (r, g, b)), m| {
                    let (numstr, color) = m.as_str().split_once(' ').unwrap();
                    let num = numstr.parse::<i32>().unwrap();
                    (
                        // accumulate part 1 result
                        if game_id.is_some() {
                            match color {
                                "r" if num <= RED => game_id,
                                "g" if num <= GREEN => game_id,
                                "b" if num <= BLUE => game_id,
                                _ => None,
                            }
                        } else {
                            None
                        },
                        // accumulate part 2 result
                        match color {
                            "r" => (r.max(num), g, b),
                            "g" => (r, g.max(num), b),
                            "b" => (r, g, b.max(num)),
                            _ => unreachable!(),
                        },
                    )
                },
            );

            (p1 + game_id.unwrap_or(0), r * g * b + p2)
        });

    (p1.to_string(), p2.to_string())
}

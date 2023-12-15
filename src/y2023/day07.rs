use crate::Solver;
use std::convert::identity;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        let _x = input
            .lines()
            .map(|s| s.split_once(' '))
            .filter_map(identity)
            .map(|(_hand, _bid)| {
                // println!("hand={hand} bid={bid}");
                0
            })
            .collect::<Vec<_>>();
        (String::new(), String::new())
    }
}

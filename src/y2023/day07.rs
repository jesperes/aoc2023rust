use std::convert::identity;

pub fn solve(input: &str) -> (i32, i32) {
    let _x = input
        .lines()
        .map(|s| s.split_once(' '))
        .filter_map(identity)
        .map(|(hand, bid)| {
            // println!("hand={hand} bid={bid}");
            0
        })
        .collect::<Vec<_>>();
    (0, 0)
}

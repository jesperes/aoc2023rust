use itertools::Itertools;

use crate::Solver;

// extern crate geo;
// extern crate line_intersection;

pub struct Solution;

type ResultType = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

impl Hailstone {
    fn intersects_at_xy(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let a = self;
        let b = other;

        let a_x = a.x as f64;
        let a_y = a.y as f64;
        let a_dx = a.dx as f64;
        let a_dy = a.dy as f64;

        let b_x = b.x as f64;
        let b_y = b.y as f64;
        let b_dx = b.dx as f64;
        let b_dy = b.dy as f64;

        let dx = b_x - a_x;
        let dy = b_y - a_y;
        let det = b_dx * a_dy - b_dy * a_dx;
        if det != 0f64 {
            let u = (dy * b_dx - dx * b_dy) / det;
            let v = (dy * a_dx - dx * a_dy) / det;
            if u >= 0f64 && v >= 0f64 {
                let res_a_x = a_x + a_dx * u;
                let res_a_y = a_y + a_dy * u;
                Some((res_a_x, res_a_y))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Solver<ResultType, ResultType> for Solution {
    fn solve(&self, input: &str) -> (ResultType, ResultType) {
        solve(input)
    }
}

fn solve(input: &str) -> (ResultType, ResultType) {
    let hailstones = parse(input);
    let p1 = solve_p1(&hailstones, (200000000000000f64, 400000000000000f64));
    (p1, 0)
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (x, y, z, dx, dy, dz) = line
                .split(|s| ", @".contains(s))
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Hailstone {
                x,
                y,
                z,
                dx,
                dy,
                dz,
            }
        })
        .collect_vec()
}

fn solve_p1(hailstones: &[Hailstone], test_area: (f64, f64)) -> usize {
    let (lower, upper) = test_area;
    hailstones
        .iter()
        .cartesian_product(hailstones.iter())
        .filter(|(a, b)| a < b)
        .filter_map(|(a, b)| a.intersects_at_xy(b))
        .filter(|&(x, y)| x >= lower && x <= upper && y >= lower && y <= upper)
        .count()
}

#[test]
fn test_ex1() {
    let ex = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    let hailstones = parse(ex);

    assert_eq!(2, solve_p1(&hailstones, (7f64, 27f64)));
}

use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};

use crate::Solver;
pub struct Solution;
impl Solver<i64, i64> for Solution {
    fn solve(&self, input: &str) -> (i64, i64) {
        solve(input)
    }
}

type RowCol = (i64, i64);

fn solve(input: &str) -> (i64, i64) {
    let grid = parse(input);
    let p1 = solve_p1(&grid, 64);
    let p2 = solve_p2(&grid);
    (p1, p2)
}

fn parse(input: &str) -> (HashMap<RowCol, char>, RowCol, RowCol) {
    let mut grid: HashMap<RowCol, char> = HashMap::new();
    let mut start: Option<RowCol> = None;

    let rows = input.lines().count() as i64;
    let cols = input.lines().next().unwrap().len() as i64;

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let pos = (row as i64, col as i64);
            match c {
                'S' => {
                    start = Some(pos);
                    grid.insert(pos, '.');
                }
                c if c == '#' || c == '.' => {
                    grid.insert(pos, c);
                }
                _ => {}
            }
        }
    }

    (grid, start.unwrap(), (rows, cols))
}

fn solve_p1(grid: &(HashMap<RowCol, char>, RowCol, RowCol), max_depth: i64) -> i64 {
    bfs(grid, max_depth)
}

fn bfs(grid: &(HashMap<RowCol, char>, RowCol, RowCol), max_depth: i64) -> i64 {
    let (map, start, (rows, cols)) = grid;
    assert_eq!(rows, cols);

    let mut queue: VecDeque<(i64, RowCol)> = VecDeque::new();
    let mut visited: HashSet<RowCol> = HashSet::new();
    let mut total = 0;
    let parity = max_depth % 2;
    queue.push_back((0, *start));

    while let Some((depth, pos)) = queue.pop_front() {
        // println!("{i} pos={:?} depth={:?} max_depth={max_depth}", pos, depth);
        if depth > max_depth {
            break;
        } else if visited.contains(&pos) {
            continue;
        } else if depth % 2 == parity {
            total += 1;
        }

        visited.insert(pos);

        let (row, col) = pos;
        [
            (row - 1, col),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col),
        ]
        .iter()
        .filter(|nbr| {
            let (nbr_row, nbr_col) = nbr;
            let nbr_pos = (nbr_row.rem_euclid(*rows), nbr_col.rem_euclid(*cols));
            let (r, c) = nbr_pos;
            assert!(r >= 0 && c >= 0 && r < *rows && c < *cols);
            !visited.contains(*nbr) && map.get(&nbr_pos).unwrap() != &'#'
        })
        .for_each(|nbr_pos| queue.push_back((depth + 1, *nbr_pos)));
    }

    total
}

// For part 2, we need to take a very large (26501365) number of steps in a
// infinite grid. The solution can be computed using a quadratic sequence.
//
// Inspired by
// https://github.com/mebeim/aoc/blob/master/2023/original_solutions/day21.py
fn solve_p2(grid: &(HashMap<RowCol, char>, RowCol, RowCol)) -> i64 {
    let (_map, _start, (rows, cols)) = grid;
    assert_eq!(rows, cols);

    let constant = 26501365;
    let modulo = constant % rows;

    let a = bfs(grid, modulo);
    let b = bfs(grid, modulo + rows);
    let c = bfs(grid, modulo + 2 * rows);

    let first_diff1 = b - a;
    let first_diff2 = c - b;
    let second_diff = first_diff2 - first_diff1;

    // https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html
    let a0 = second_diff.div_floor(2);
    let b0 = first_diff1 - 3 * a0;
    let c0 = a - b0 - a0;
    let x = constant.div_ceil(*rows);
    a0 * (x * x) + b0 * x + c0
}

#[test]
fn test1() {
    let x: i32 = -1;
    assert_eq!(3, x.rem_euclid(4));
}

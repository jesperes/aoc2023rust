use std::collections::BTreeSet;

use hashbrown::HashMap;

use crate::Solver;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        (solve_p1(input).to_string(), solve_p2(input).to_string())
    }
}

type RowCol = (i32, i32);
type Grid = HashMap<RowCol, char>;
type QueueElem = (RowCol, Dir);
type EnergizedMap = HashMap<RowCol, Dir>;

type Dir = i32;
const UP: i32 = 1 << 0;
const DOWN: i32 = 1 << 1;
const LEFT: i32 = 1 << 2;
const RIGHT: i32 = 1 << 3;

enum MirrorAction {
    Reflect(Dir),
    Passthrough,
    Split(Dir, Dir),
}

fn solve_p1(input: &str) -> usize {
    let (grid, dims) = parse_into_grid(input);
    project_beam(&dims, ((0, 0), RIGHT), &grid)
}

fn solve_p2(input: &str) -> usize {
    let (grid, dims) = parse_into_grid(input);
    let (rows, cols) = dims;

    let left_edge = (0..rows).map(|row| ((row, 0), RIGHT));
    let right_edge = (0..rows).map(|r| ((r, cols - 1), LEFT));
    let upper_edge = (0..cols).map(|c| ((0, c), DOWN));
    let bottom_edge = (0..cols).map(|c| ((rows - 1, c), UP));

    left_edge
        .chain(right_edge)
        .chain(bottom_edge)
        .chain(upper_edge)
        .map(|start| project_beam(&dims, start, &grid))
        .max()
        .unwrap()
}

fn parse_into_grid(input: &str) -> (Grid, (i32, i32)) {
    let mut grid: Grid = HashMap::new();

    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().len() as i32;
    let dims = (rows, cols);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '|' || c == '-' || c == '/' || c == '\\' {
                grid.insert((row as i32, col as i32), c);
            }
        }
    }
    (grid, dims)
}

fn next_pos(pos: (i32, i32), direction: &Dir) -> (i32, i32) {
    let (row, col) = pos;
    match *direction {
        RIGHT => (row, col + 1),
        DOWN => (row + 1, col),
        UP => (row - 1, col),
        LEFT => (row, col - 1),
        _ => unreachable!(),
    }
}

fn in_cave(pos: &(i32, i32), dims: &(i32, i32)) -> bool {
    let (row, col) = pos;
    let (rows, cols) = dims;
    *row >= 0 && row < rows && *col >= 0 && col < cols
}

/// Project a beam starting at `pos`
fn project_beam(dims: &(i32, i32), start: ((i32, i32), i32), grid: &Grid) -> usize {
    let mut q: Vec<QueueElem> = Vec::new();
    let mut energized_map = EnergizedMap::new();
    q.push(start);

    while let Some((pos, orig_direction)) = q.pop() {
        if !in_cave(&pos, &dims) {
            continue;
        }

        let energy = energized_map.get(&pos).unwrap_or(&0);
        if energy & orig_direction != 0 {
            // loop
            continue;
        } else {
            energized_map.insert(pos, energy | orig_direction);
        }

        // Check if we hit the cave wall

        let dirs: MirrorAction = match (grid.get(&pos), orig_direction) {
            // Passing through empty space or splitters
            (None, _) => MirrorAction::Passthrough,
            (Some('-'), LEFT) => MirrorAction::Passthrough,
            (Some('-'), RIGHT) => MirrorAction::Passthrough,
            (Some('|'), UP) => MirrorAction::Passthrough,
            (Some('|'), DOWN) => MirrorAction::Passthrough,

            // Splitting the beam
            (Some('-'), UP) => MirrorAction::Split(LEFT, RIGHT),
            (Some('-'), DOWN) => MirrorAction::Split(LEFT, RIGHT),

            (Some('|'), LEFT) => MirrorAction::Split(UP, DOWN),
            (Some('|'), RIGHT) => MirrorAction::Split(UP, DOWN),

            // Reflections
            (Some('\\'), RIGHT) => MirrorAction::Reflect(DOWN),
            (Some('\\'), UP) => MirrorAction::Reflect(LEFT),
            (Some('\\'), DOWN) => MirrorAction::Reflect(RIGHT),
            (Some('\\'), LEFT) => MirrorAction::Reflect(UP),
            (Some('/'), RIGHT) => MirrorAction::Reflect(UP),
            (Some('/'), UP) => MirrorAction::Reflect(RIGHT),
            (Some('/'), DOWN) => MirrorAction::Reflect(LEFT),
            (Some('/'), LEFT) => MirrorAction::Reflect(DOWN),

            _ => unreachable!(),
        };

        // TODO do not insert elements here which we know are outside
        // the grid
        match dirs {
            MirrorAction::Reflect(reflect_dir) => {
                q.push((next_pos(pos, &reflect_dir), reflect_dir));
            }
            MirrorAction::Passthrough => {
                q.push((next_pos(pos, &orig_direction), orig_direction));
            }
            MirrorAction::Split(dir1, dir2) => {
                q.push((next_pos(pos, &dir1), dir1));
                q.push((next_pos(pos, &dir2), dir2));
            }
        }
    }

    energized_map.len()
}

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

    // print_grid(&grid, &dims);
    project_beam(&dims, &grid)
    // print_energized_grid(&energized, &dims);
}

fn solve_p2(_input: &str) -> i32 {
    0
}

fn print_energized_grid(grid: &HashMap<RowCol, Vec<Dir>>, dims: &(i32, i32)) {
    let (rows, cols) = dims;
    println!("Energized grid ({rows}x{cols})");
    for row in 0..*rows {
        print!("{:3} ", row);
        for col in 0..*cols {
            let pos = (row, col);
            if grid.contains_key(&pos) {
                let v = grid.get(&pos).unwrap();
                if v.len() == 1 {
                    let c = match v[0] {
                        UP => '\u{25b5}',
                        DOWN => '\u{25bf}',
                        LEFT => '\u{25c3}',
                        RIGHT => '\u{25b9}',
                        _ => unreachable!(),
                    };
                    print!("{c}");
                } else if v.len() > 1 {
                    print!("#");
                } else {
                    unreachable!();
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_grid(grid: &Grid, dims: &(i32, i32)) {
    let (rows, cols) = dims;
    println!("Grid ({rows}x{cols})");
    for row in 0..*rows {
        print!("{:3} ", row);
        for col in 0..*cols {
            print!("{}", grid.get(&(row, col)).unwrap_or(&'.'));
        }
        println!();
    }
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
fn project_beam(dims: &(i32, i32), grid: &Grid) -> usize {
    let mut q: BTreeSet<QueueElem> = BTreeSet::new();
    let mut energized_map = EnergizedMap::new();
    let start = ((0, 0), RIGHT);
    q.insert(start);

    while let Some((pos, orig_direction)) = q.pop_first() {
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

        match dirs {
            MirrorAction::Reflect(reflect_dir) => {
                q.insert((next_pos(pos, &reflect_dir), reflect_dir));
            }
            MirrorAction::Passthrough => {
                q.insert((next_pos(pos, &orig_direction), orig_direction));
            }
            MirrorAction::Split(dir1, dir2) => {
                q.insert((next_pos(pos, &dir1), dir1));
                q.insert((next_pos(pos, &dir2), dir2));
            }
        }
    }

    energized_map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ex = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";
        assert_eq!(46, solve_p1(&ex));
    }
}

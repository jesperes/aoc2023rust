use grid::Grid;
use hashbrown::HashSet;

type Platform = Grid<char>;
enum Direction {
    North,
    West,
    South,
    East,
}

fn can_move_to(platform: &Platform, row: i32, col: i32) -> bool {
    row >= 0
        && row < platform.rows() as i32
        && col >= 0
        && col < platform.cols() as i32
        && platform[(row as usize, col as usize)] == '.'
}

fn tilt(platform: &mut Platform, dir: Direction, _cache: &mut HashSet<Platform>) {
    let rows = platform.rows();
    let cols = platform.cols();
    for row in 0..rows {
        for col in 0..cols {
            // invert the row/col when tilting south or east
            let (row0, col0): (usize, usize) = match dir {
                Direction::North => (row, col),
                Direction::West => (row, col),
                Direction::South => {
                    let inv_row = ((rows as i32) - (row as i32) - 1) as usize;
                    (inv_row, col)
                }
                Direction::East => {
                    let inv_col = (cols as i32 - col as i32 - 1) as usize;
                    (row, inv_col)
                }
            };

            if platform[(row0, col0)] == 'O' {
                let (delta_row, delta_col) = match dir {
                    Direction::North => (-1, 0),
                    Direction::West => (0, -1),
                    Direction::East => (0, 1),
                    Direction::South => (1, 0),
                };
                let mut next_row = row0 as i32;
                let mut next_col = col0 as i32;
                let mut maybe_next_row = next_row + delta_row;
                let mut maybe_next_col = next_col + delta_col;

                while can_move_to(platform, maybe_next_row, maybe_next_col) {
                    next_row = maybe_next_row;
                    next_col = maybe_next_col;
                    maybe_next_row += delta_row;
                    maybe_next_col += delta_col;
                }

                platform[(row0, col0)] = '.';
                platform[(next_row as usize, next_col as usize)] = 'O';
            }
        }
    }
}

fn get_load(platform: &Platform) -> usize {
    let rows = platform.rows();
    let cols = platform.cols();
    let mut load = 0;
    for row in 0..rows {
        for col in 0..cols {
            if platform[(row, col)] == 'O' {
                load += rows - row;
            }
        }
    }
    return load;
}

pub fn solve(input: &str) -> (usize, i64) {
    let mut cache = HashSet::new();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut platform = Platform::new(rows, cols);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            platform[(row, col)] = c;
        }
    }

    let mut p1: Option<usize> = None;
    let start = std::time::Instant::now();
    for cycle in 0..1_000_000_000 {
        tilt(&mut platform, Direction::North, &mut cache);
        // store p1 on the first round
        p1.get_or_insert_with(|| get_load(&platform));
        tilt(&mut platform, Direction::West, &mut cache);
        tilt(&mut platform, Direction::South, &mut cache);
        tilt(&mut platform, Direction::East, &mut cache);

        if cycle % 1000 == 0 {
            let elapsed = start.elapsed();
            let ns_per_cycle = elapsed.as_nanos() / (cycle + 1);
            let remaining_cycles = 1_000_000_000 - (cycle + 1);
            let remaining_ns = remaining_cycles * ns_per_cycle;
            let proj_hours = (remaining_ns as f64 / 1_000_000_000f64) / 3600f64;
            println!(
                "Completed {cycle} cycles in {} ns/cycle, projected time to completion: {:.2} hours",
                ns_per_cycle, proj_hours
            );
        }
    }

    let p2 = get_load(&platform);
    (p1.unwrap(), p2 as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1_test() {
        let ex = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!((136, 0), solve(&ex));
    }
}

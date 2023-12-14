use grid::Grid;
use hashbrown::HashMap;

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

fn tilt(platform: &mut Platform, dir: Direction) {
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
    platform
        .indexed_iter()
        .filter_map(|((row, _), c)| {
            if *c == 'O' {
                Some(platform.rows() - row)
            } else {
                None
            }
        })
        .sum()
}

fn get_key(platform: &Platform) -> String {
    let rows = platform.rows();
    let cols = platform.cols();
    let mut key = String::new();
    for row in 0..rows {
        for col in 0..cols {
            key.push(platform[(row, col)]);
        }
    }
    key
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut cache: HashMap<String, i32> = HashMap::new();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut platform = Platform::new(rows, cols);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            platform[(row, col)] = c;
        }
    }

    let mut p1: Option<usize> = None;
    let p2;
    let mut total_cycles = 1_000_000_000;
    let mut cycle = 1;
    let mut enable_caching = true;
    loop {
        tilt(&mut platform, Direction::North);
        // store p1 on the first round
        p1.get_or_insert_with(|| get_load(&platform));
        tilt(&mut platform, Direction::West);
        tilt(&mut platform, Direction::South);
        tilt(&mut platform, Direction::East);

        if cycle == total_cycles {
            p2 = Some(get_load(&platform));
            break;
        }

        if enable_caching {
            let key = get_key(&platform);
            if cache.contains_key(&key) {
                let period_start = cache.get(&key).unwrap();
                let period_len = cycle - period_start;
                let remaining_cycles = total_cycles - cycle;
                let remaining_whole_periods = remaining_cycles.div_floor(period_len);
                total_cycles -= remaining_whole_periods * period_len;
                enable_caching = false;
            } else {
                cache.insert(key, cycle);
            }
        }

        cycle += 1;
    }

    (p1.unwrap(), p2.unwrap())
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

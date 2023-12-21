use crate::{utils::astar::*, Solver};
pub struct Solution;
impl Solver<usize, usize> for Solution {
    fn solve(&self, input: &str) -> (usize, usize) {
        let p1 = Config {
            max_cnt: 3,
            min_cnt: None,
        };
        let p2 = Config {
            max_cnt: 10,
            min_cnt: Some(4),
        };
        (do_solve(input, p1), do_solve(input, p2))
    }
}

type RowCol = (i32, i32);

type Dir = u8;
const UP: u8 = 1;
const LEFT: u8 = 2;
const RIGHT: u8 = 3;
const DOWN: u8 = 4;
const UNSPECIFIED: u8 = 5;

struct Config {
    // Maximum number of steps in a straight line before we must turn
    max_cnt: i32,
    // Minimum number of steps in a straight line before we are allowed to make a turn
    min_cnt: Option<i32>,
}

struct HeatMap<'a> {
    data: &'a [u8],
    dy: i32,
    limits: RowCol,
    goal: RowCol,
    config: &'a Config,
}

impl<'a> HeatMap<'a> {
    fn new(input: &'a str, config: &'a Config) -> HeatMap<'a> {
        let cols = input.lines().next().unwrap().len() as i32;
        let rows = input.lines().count() as i32;

        HeatMap {
            data: input.as_bytes(),
            dy: cols + 1,
            limits: (rows, cols),
            goal: (rows - 1, cols - 1),
            config,
        }
    }

    /// Returns the heat loss at a given position
    fn heat_loss(&self, pos: RowCol) -> i32 {
        let (row, col) = pos;
        (self.data[(row * self.dy + col) as usize] - b'0') as i32
    }
}

struct CrucibleState<'a> {
    heat_map: &'a HeatMap<'a>,
    pos: RowCol,

    // Track straight lines
    dir_count: i32,
    current_dir: Dir,

    // The total cost of reaching this node, including the cost of this node
    total_cost: usize,
}

impl<'a> CrucibleState<'a> {
    fn new(heat_map: &'a HeatMap<'a>, straight_dir: Dir, straight_count: i32) -> CrucibleState<'a> {
        CrucibleState {
            heat_map,
            pos: (0, 0),
            total_cost: 0,
            dir_count: straight_count,
            current_dir: straight_dir,
        }
    }
}

impl<'a> SearchState for CrucibleState<'a> {
    // We need to distinguish search states depending on which direction we
    // reached them in, and how far we have travelled in a straight line when
    // doing so.
    type Key = u32;
    type Iter = std::vec::IntoIter<CrucibleState<'a>>;

    fn key(&self) -> Self::Key {
        // The key is used for hash lookups a lot, so compress it into a 32-bit integer.
        let (row, col) = self.pos;
        let r = ((row as u32) & 0xff) << 24;
        let c = ((col as u32) & 0xff) << 16;
        let d = ((self.current_dir as u32) & 0xff) << 8;
        let n = (self.dir_count as u32) & 0xff;
        r | c | d | n
    }

    fn is_goal(&self) -> bool {
        if let Some(min_straight_count) = self.heat_map.config.min_cnt {
            self.pos == self.heat_map.goal && self.dir_count >= min_straight_count
        } else {
            self.pos == self.heat_map.goal
        }
    }

    fn cost(&self) -> usize {
        self.total_cost
    }

    fn heuristic(&self) -> usize {
        (self.pos.0.abs_diff(self.heat_map.goal.0) + self.pos.1.abs_diff(self.heat_map.goal.1))
            as usize
    }

    fn next_states(self) -> Self::Iter {
        //use Dir::*;

        [
            (UP, (-1, 0)),
            (DOWN, (1, 0)),
            (LEFT, (0, -1)),
            (RIGHT, (0, 1)),
        ]
        .into_iter()
        .filter_map(|(dir, (delta_row, delta_col))| {
            let count = self.dir_count;
            let curr_dir = self.current_dir;

            if (curr_dir == dir && count >= self.heat_map.config.max_cnt)
                || (curr_dir != dir
                    && curr_dir != UNSPECIFIED
                    && count < self.heat_map.config.min_cnt.unwrap_or(0))
                || ((curr_dir == UP && dir == DOWN)
                    || (curr_dir == DOWN && dir == UP)
                    || (curr_dir == RIGHT && dir == LEFT)
                    || (curr_dir == LEFT && dir == RIGHT))
            {
                None
            } else {
                let (rows, cols) = self.heat_map.limits;
                let (row, col) = self.pos;
                let (nrow, ncol) = (row + delta_row, col + delta_col);
                let dir_count = if dir == curr_dir {
                    self.dir_count + 1
                } else {
                    1
                };
                if nrow >= 0 && nrow < rows && ncol >= 0 && ncol < cols {
                    let cost = self.heat_map.heat_loss((nrow, ncol));
                    Some((dir, dir_count, (nrow, ncol), cost))
                } else {
                    None
                }
            }
        })
        .map(|(dir, dir_count, pos, cost)| CrucibleState {
            heat_map: self.heat_map,
            pos,
            total_cost: self.total_cost + cost as usize,
            dir_count,
            current_dir: dir,
        })
        .collect::<Vec<_>>()
        .into_iter()
    }
}

fn do_solve(input: &str, config: Config) -> usize {
    let heat_map = HeatMap::new(input, &config);
    solve(CrucibleState::new(&heat_map, UNSPECIFIED, 0))
        .unwrap()
        .cost()
}

use crate::{utils::astar::*, Solver};
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &str) -> (String, String) {
        let p1 = Config {
            max_cnt: 3,
            min_cnt: None,
        };
        let p2 = Config {
            max_cnt: 10,
            min_cnt: Some(4),
        };
        (
            do_solve(input, p1).to_string(),
            do_solve(input, p2).to_string(),
        )
    }
}

type RowCol = (i32, i32);

#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
    Unspecified,
}

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
    fn heat_loss(&self, pos: RowCol) -> u8 {
        let (row, col) = pos;
        self.data[(row * self.dy + col) as usize] - b'0'
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
    type Key = (RowCol, Dir, i32);

    type Iter = std::vec::IntoIter<CrucibleState<'a>>;

    fn key(&self) -> Self::Key {
        (self.pos, self.current_dir, self.dir_count)
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
        use Dir::*;

        let v = vec![
            (Up, (-1, 0)),
            (Down, (1, 0)),
            (Left, (0, -1)),
            (Right, (0, 1)),
        ];

        v.into_iter()
            .filter_map(|(dir, (delta_row, delta_col))| {
                let count = self.dir_count;
                let curr_dir = self.current_dir;

                // First, check the conditions on which directions we are allowed to take
                if (curr_dir == dir && count >= self.heat_map.config.max_cnt)
                    || (curr_dir != dir
                        && curr_dir != Unspecified
                        && count < self.heat_map.config.min_cnt.unwrap_or(0))
                    || ((curr_dir == Up && dir == Down)
                        || (curr_dir == Down && dir == Up)
                        || (curr_dir == Right && dir == Left)
                        || (curr_dir == Left && dir == Right))
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
                        Some((dir, dir_count, (nrow, ncol)))
                    } else {
                        None
                    }
                }
            })
            .map(|(dir, dir_count, pos)| CrucibleState {
                heat_map: self.heat_map,
                pos,
                total_cost: self.total_cost + self.heat_map.heat_loss(pos) as usize,
                dir_count,
                current_dir: dir,
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

fn do_solve(input: &str, config: Config) -> usize {
    let heat_map = HeatMap::new(input, &config);
    solve(CrucibleState::new(&heat_map, Dir::Unspecified, 0))
        .unwrap()
        .cost()
}

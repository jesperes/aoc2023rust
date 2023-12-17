use crate::{utils::astar::*, Solver};
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        (
            // max_straight_count = 4
            do_solve(
                input,
                Config {
                    max_straight_count: 3,
                    min_straight_count: None,
                },
            )
            .to_string(),
            // max_straight_count = 10, min_straight_count = 4
            do_solve(
                input,
                Config {
                    max_straight_count: 10,
                    min_straight_count: Some(4),
                },
            )
            .to_string(),
        )
    }
}

type RowCol = (i32, i32);

struct Config {
    max_straight_count: i32,
    min_straight_count: Option<i32>,
}

#[derive(Debug)]
struct HeatMap<'a> {
    data: &'a [u8],
    dy: i32,
    limits: RowCol,
}

impl<'a> HeatMap<'a> {
    fn new(input: &'a str) -> HeatMap<'a> {
        let cols = input.lines().next().unwrap().len() as i32;
        let rows = input.lines().count() as i32;

        HeatMap {
            data: input.as_bytes(),
            dy: cols + 1,
            limits: (rows, cols),
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
    goal: RowCol,

    // Track straight lines
    dir_count: i32,
    current_dir: Direction,

    // The total cost of reaching this node, including the cost of this node
    total_cost: usize,

    // Part 1 and 2 have slightly different semantics
    config: &'a Config,
}

impl<'a> CrucibleState<'a> {
    fn new(
        heat_map: &'a HeatMap<'a>,
        straight_dir: Direction,
        straight_count: i32,
        config: &'a Config,
    ) -> CrucibleState<'a> {
        let (rows, cols) = heat_map.limits;
        CrucibleState {
            heat_map,
            pos: (0, 0),
            goal: (rows - 1, cols - 1),
            total_cost: 0,
            dir_count: straight_count,
            current_dir: straight_dir,
            config,
        }
    }
}

impl<'a> SearchState for CrucibleState<'a> {
    // We need to distinguish search states depending on which direction we
    // reached them in, and how far we have travelled in a straight line when
    // doing so.
    type Key = (RowCol, Direction, i32);

    type Iter = CrucibleStateIterator<'a>;

    fn key(&self) -> Self::Key {
        (self.pos, self.current_dir, self.dir_count)
    }

    fn is_goal(&self) -> bool {
        if let Some(min_straight_count) = self.config.min_straight_count {
            self.pos == self.goal && self.dir_count >= min_straight_count
        } else {
            self.pos == self.goal
        }
    }

    fn cost(&self) -> usize {
        self.total_cost
    }

    fn heuristic(&self) -> usize {
        (self.pos.0.abs_diff(self.goal.0) + self.pos.1.abs_diff(self.goal.1)) as usize
    }

    fn next_states(self) -> Self::Iter {
        CrucibleStateIterator {
            state: self,
            dir: Direction::first(),
        }
    }
}

/// Used in the iterator to track the current direction
#[derive(PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
    None,
}

impl Direction {
    fn first() -> Direction {
        Direction::Up
    }

    fn bump(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => None,
            None => None,
        };
    }
}

struct CrucibleStateIterator<'a> {
    state: CrucibleState<'a>,
    dir: Direction,
}

impl<'a> Iterator for CrucibleStateIterator<'a> {
    type Item = CrucibleState<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = self.state.pos;
        let (rows, cols) = self.state.heat_map.limits;

        if self.dir == Direction::None {
            return None;
        }

        let maybe_next_pos = if (self.dir == Direction::Up
            && self.state.current_dir == Direction::Down)
            || (self.dir == Direction::Down && self.state.current_dir == Direction::Up)
            || (self.dir == Direction::Left && self.state.current_dir == Direction::Right)
            || (self.dir == Direction::Right && self.state.current_dir == Direction::Left)
        {
            // No reverse direction
            None
        } else if self.state.dir_count < self.state.config.min_straight_count.unwrap_or(0)
            && self.state.current_dir != Direction::None
            && self.dir != self.state.current_dir
        {
            None
        } else if self.dir == self.state.current_dir
            && self.state.dir_count >= self.state.config.max_straight_count
        {
            None
        } else {
            match self.dir {
                // Direction::None => return None,
                Direction::Up if row > 0 => Some((row - 1, col)),
                Direction::Left if col > 0 => Some((row, col - 1)),
                Direction::Down if row < rows - 1 => Some((row + 1, col)),
                Direction::Right if col < cols - 1 => Some((row, col + 1)),
                _ => None,
            }
        };

        // Count number of steps we have taken in the same direction
        let new_dir_count = if self.dir == self.state.current_dir {
            self.state.dir_count + 1
        } else {
            1
        };

        let new_dir = self.dir;

        self.dir.bump();
        if let Some(next_pos) = maybe_next_pos {
            let (next_row, next_col) = next_pos;
            Some(CrucibleState {
                heat_map: self.state.heat_map,
                pos: (next_row, next_col),
                total_cost: self.state.total_cost
                    + self.state.heat_map.heat_loss(next_pos) as usize,
                goal: self.state.goal,
                dir_count: new_dir_count,
                current_dir: new_dir,
                config: &self.state.config,
            })
        } else {
            self.next()
        }
    }
}

fn do_solve(input: &str, config: Config) -> usize {
    let heat_map = HeatMap::new(input);
    solve(CrucibleState::new(&heat_map, Direction::None, 0, &config))
        .unwrap()
        .cost()
}

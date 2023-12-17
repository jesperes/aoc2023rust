use crate::{utils::astar::*, Solver};
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        (solve_p1(input).to_string(), String::new())
    }
}
type RowCol = (i32, i32);

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

/// Search state
struct CrucibleState<'a> {
    heat_map: &'a HeatMap<'a>,
    pos: RowCol,
    goal: RowCol,

    // The total cost of reaching this node, including the cost of this node
    total_cost: usize,
}

impl<'a> CrucibleState<'a> {
    fn new(heat_map: &'a HeatMap<'a>) -> CrucibleState<'a> {
        let (rows, cols) = heat_map.limits;
        CrucibleState {
            heat_map,
            pos: (0, 0),
            goal: (rows - 1, cols - 1),
            total_cost: 0,
        }
    }
}

impl<'a> SearchState for CrucibleState<'a> {
    type Key = RowCol;

    type Iter = CrucibleStateIterator<'a>;

    fn key(&self) -> Self::Key {
        self.pos
    }

    fn is_goal(&self) -> bool {
        self.pos == self.goal
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

        let maybe_next_pos = match self.dir {
            Direction::None => return None,
            Direction::Up if row > 0 => Some((row - 1, col)),
            Direction::Left if col > 0 => Some((row, col - 1)),
            Direction::Down if row < rows - 1 => Some((row + 1, col)),
            Direction::Right if col < cols - 1 => Some((row, col + 1)),
            _ => None,
        };

        if let Some(next_pos) = maybe_next_pos {
            let (next_row, next_col) = next_pos;
            self.dir.bump();
            Some(CrucibleState {
                heat_map: self.state.heat_map,
                pos: (next_row, next_col),
                total_cost: self.state.total_cost
                    + self.state.heat_map.heat_loss(next_pos) as usize,
                goal: self.state.goal,
            })
        } else {
            self.dir.bump();
            self.next()
        }
    }
}

pub fn solve_p1(input: &str) -> usize {
    let heat_map = HeatMap::new(input);
    let end_state = solve(CrucibleState::new(&heat_map)).unwrap();
    end_state.cost()
}

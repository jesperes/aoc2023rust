use crate::{utils::astar::*, Solver};
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        (solve_p1(input).to_string(), solve_p2(input).to_string())
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

    // Track straight lines
    dir_count: i32,
    current_dir: Direction,

    // The total cost of reaching this node, including the cost of this node
    total_cost: usize,

    // Part 1 and 2 have slightly different semantics
    part: i32,
}

impl<'a> CrucibleState<'a> {
    fn new(
        heat_map: &'a HeatMap<'a>,
        straight_dir: Direction,
        straight_count: i32,
        part: i32,
    ) -> CrucibleState<'a> {
        let (rows, cols) = heat_map.limits;
        CrucibleState {
            heat_map,
            pos: (0, 0),
            goal: (rows - 1, cols - 1),
            total_cost: 0,
            dir_count: straight_count,
            current_dir: straight_dir,
            part: part,
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
#[derive(PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
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

// 995 too low

impl<'a> Iterator for CrucibleStateIterator<'a> {
    type Item = CrucibleState<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = self.state.pos;
        let (rows, cols) = self.state.heat_map.limits;

        // First check that we don't reverse direction
        let maybe_next_pos = if (self.dir == Direction::Up
            && self.state.current_dir == Direction::Down)
            || (self.dir == Direction::Down && self.state.current_dir == Direction::Up)
            || (self.dir == Direction::Left && self.state.current_dir == Direction::Right)
            || (self.dir == Direction::Right && self.state.current_dir == Direction::Left)
        {
            None
        } else if self.dir == self.state.current_dir && self.state.dir_count >= 2 {
            // No more than 3 steps in the same direction
            None
        } else {
            match self.dir {
                Direction::None => return None,
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
            0
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
                part: self.state.part,
            })
        } else {
            self.next()
        }
    }
}

pub fn solve_p1(input: &str) -> usize {
    let heat_map = HeatMap::new(input);
    solve(CrucibleState::new(&heat_map, Direction::None, 0, 1))
        .unwrap()
        .cost()
}

pub fn solve_p2(input: &str) -> usize {
    let heat_map = HeatMap::new(input);
    solve(CrucibleState::new(&heat_map, Direction::None, 0, 2))
        .unwrap()
        .cost()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ex = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        assert_eq!(102, solve_p1(ex));
    }
}

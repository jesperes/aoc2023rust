use std::fmt;

use derivative::Derivative;
use hashbrown::HashMap;

use par_dfs::sync::{FastDfs, FastNode};

use crate::Solver;

type RowCol = (i32, i32);
type ResultType = i64;
type Grid = HashMap<RowCol, char>;

#[derive(Clone, Derivative)]
#[derivative(Hash, Eq, PartialEq)]
struct Node<'a> {
    pos: RowCol,
    #[derivative(Hash = "ignore", PartialEq = "ignore")]
    depth: usize,
    #[derivative(Hash = "ignore", PartialEq = "ignore")]
    grid: &'a Grid,
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={:?} depth={:?}", self.pos, self.depth)
    }
}

impl<'a> FastNode for Node<'a> {
    type Error = std::convert::Infallible;

    fn add_children<E>(&self, depth: usize, queue: &mut E) -> Result<(), Self::Error>
    where
        E: par_dfs::sync::ExtendQueue<Self, Self::Error>,
    {
        let (row, col) = self.pos;
        let grid = self.grid;

        if let Some(downhills_nbr) = match *self.grid.get(&self.pos).unwrap_or(&'#') {
            '<' => Some(Node {
                pos: (row, col - 1),
                grid,
                depth,
            }),
            '>' => Some(Node {
                pos: (row, col + 1),
                grid,
                depth,
            }),
            '^' => Some(Node {
                pos: (row - 1, col),
                grid,
                depth,
            }),
            'v' => Some(Node {
                pos: (row + 1, col),
                grid,
                depth,
            }),
            _ => None,
        } {
            queue.add(Ok(downhills_nbr));
        } else {
            queue.add_all(
                [
                    (row - 1, col),
                    (row + 1, col),
                    (row, col - 1),
                    (row, col + 1),
                ]
                .iter()
                .filter_map(|nbr| {
                    let c = *self.grid.get(nbr).unwrap_or(&'#');
                    if c == '#' {
                        None
                    } else {
                        Some(Ok(Node {
                            pos: *nbr,
                            grid,
                            depth,
                        }))
                    }
                }),
            );
        }

        Ok(())
    }
}

pub struct Solution;
impl Solver<ResultType, ResultType> for Solution {
    fn solve(&self, input: &str) -> (ResultType, ResultType) {
        solve(input)
    }
}

fn solve(input: &str) -> (ResultType, ResultType) {
    let mut grid: Grid = HashMap::new();
    let mut start: RowCol = (0, 0);
    let mut _end: RowCol = (0, 0);

    let rows = input.lines().count() as i32;
    let _cols = input.lines().next().unwrap().len() as i32;

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid.insert((row as i32, col as i32), c);
        }
    }

    for ((row, col), c) in &grid {
        if *row == rows - 1 && *c == '.' {
            _end = (*row, *col)
        } else if *row == 0 && *c == '.' {
            start = (*row, *col)
        }
    }

    // println!("rows={rows}");
    // println!("cols={cols}");
    // println!("start={:?}", start);
    // println!("end={:?}", end);

    let root = Node {
        pos: start,
        grid: &grid,
        depth: 0,
    };

    let dfs = FastDfs::<Node>::new(root, None, false);
    // println!("collecting...");

    let nodes = dfs
        .map(|node| node.unwrap())
        .map(|node| (node.pos, node))
        .collect::<HashMap<_, _>>();

    // for (k, v) in &nodes {
    //     println!("{:?}={:?}", k, v);
    // }

    // for row in 0..rows {
    //     for col in 0..cols {
    //         let pos = (row, col);
    //         let c = *grid.get(&pos).unwrap_or(&'#');
    //         let node = nodes.get(&pos);

    //         if c == '#' {
    //             print!("#")
    //         } else if let Some(_node) = node {
    //             // print!("{}", node.depth);
    //             print!("O");
    //         } else {
    //             print!("?");
    //         }
    //     }
    //     println!();
    // }

    let p1 = nodes.values().map(|node| node.depth).max().unwrap() as i64;

    // println!("p1={p1}");
    (p1, 0)
}

// #[test]
// fn test_ex1() {
//     let ex1 = "\
// #.###
// #...#
// ###.#
// #.>.#
// ###.#
// ";
//     solve(ex1);

//     let ex = "#.#####################
// #.......#########...###
// #######.#########.#.###
// ###.....#.>.>.###.#.###
// ###v#####.#v#.###.#.###
// ###.>...#.#.#.....#...#
// ###v###.#.#.#########.#
// ###...#.#.#.......#...#
// #####.#.#.#######.#.###
// #.....#.#.#.......#...#
// #.#####.#.#.#########v#
// #.#...#...#...###...>.#
// #.#.#v#######v###.###v#
// #...#.>.#...>.>.#.###.#
// #####v#.#.###v#.#.###.#
// #.....#...#...#.#.#...#
// #.#########.###.#.#.###
// #...###...#...#...#.###
// ###.###.#.###v#####v###
// #...#...#.#.>.>.#.>.###
// #.###.###.#.###.#.#v###
// #.....###...###...#...#
// #####################.#
// ";
//     solve(ex);
// }

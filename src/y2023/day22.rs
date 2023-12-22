use std::fmt::{self, Display};

use crate::Solver;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type ResultType = i32;
type CoordInt = i32;
//type Coord3D = (CoordInt, CoordInt, CoordInt);
type BrickId = i32;
// type Brick = (Coord3D, Coord3D, BrickId);
// type Tower = HashMap<BrickId, Brick>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: CoordInt,
    y: CoordInt,
    z: CoordInt,
}

impl Coord {
    fn new_from_tuple(coord: &(CoordInt, CoordInt, CoordInt)) -> Self {
        let (x, y, z) = *coord;
        Coord { x, y, z }
    }

    fn dropn(&self, n: i32) -> Coord {
        Coord {
            z: self.z - n,
            ..*self
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Brick {
    corner1: Coord,
    corner2: Coord,
    id: BrickId,
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.id_as_char(), self.corner1, self.corner2)
    }
}

impl Brick {
    fn new(id: BrickId, corner1: Coord, corner2: Coord) -> Self {
        Brick {
            id,
            corner1,
            corner2,
        }
    }

    fn lowest_point(&self) -> CoordInt {
        self.corner1.z.min(self.corner2.z)
    }

    fn dropn(&self, n: i32) -> Brick {
        Brick {
            id: self.id,
            corner1: self.corner1.dropn(n),
            corner2: self.corner2.dropn(n),
        }
    }

    /// Returns an iterator over all the invididual cubes of this brick
    fn cubes(&self) -> HashSet<Coord> {
        let mut cubes = HashSet::new();
        for x in iter_ordered_inclusive(self.corner1.x, self.corner2.x) {
            for y in iter_ordered_inclusive(self.corner1.y, self.corner2.y) {
                for z in iter_ordered_inclusive(self.corner1.z, self.corner2.z) {
                    cubes.insert(Coord::new_from_tuple(&(x, y, z)));
                }
            }
        }
        cubes
    }

    fn overlaps(&self, other: &Brick) -> bool {
        !self.cubes().is_disjoint(&other.cubes())
    }

    fn id_as_char(&self) -> char {
        (b'A' + self.id as u8) as char
    }
}

#[derive(Debug)]
struct Tower {
    bricks: HashMap<BrickId, Brick>,
}

impl Tower {
    /// Create a new tower by parsing the puzzle input
    fn new_from_input(input: &str) -> Self {
        Tower {
            bricks: input
                .lines()
                .zip(0..)
                .map(|(line, i)| {
                    let (x0, y0, z0, x1, y1, z1) = line
                        .split(|c| "~,".contains(c))
                        .map(|s| s.parse::<CoordInt>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (
                        i,
                        Brick::new(
                            i,
                            Coord::new_from_tuple(&(x0, y0, z0)),
                            Coord::new_from_tuple(&(x1, y1, z1)),
                        ),
                    )
                })
                .collect::<HashMap<_, _>>(),
        }
    }

    /// Return an iterator which iterates over all the bricks in order of
    /// lowest->highest.
    fn iter_brick_from_bottom(&self) -> std::vec::IntoIter<&Brick> {
        let mut bricks = self.bricks.values().collect_vec();
        bricks.sort_by_key(|elem| elem.lowest_point());
        bricks.into_iter()
    }

    /// Returns true if `brick` overlaps with any brick in the tower, false
    /// otherwise.
    fn overlapping_bricks(&self, other: &Brick) -> Vec<&Brick> {
        self.bricks
            .values()
            .filter(|brick| brick.overlaps(other))
            .collect::<Vec<_>>()
    }

    fn drop_all_bricks(&mut self) {
        for brick in self.iter_brick_from_bottom() {
            println!("\n## Dropping brick: {}", brick);
            for dz in 1.. {
                let dropped = brick.dropn(dz);
                if dropped.lowest_point() == 0 {
                    println!("Dropped brick {} reached bottom at {}", brick, dropped);
                    break;
                } else {
                    let overlapping_bricks = self.overlapping_bricks(&dropped);
                    if !overlapping_bricks.is_empty() {
                        println!(
                            "Dropped brick {} overlaps with existing bricks in tower at {}: {:?}",
                            brick,
                            dropped,
                            overlapping_bricks
                                .iter()
                                .map(|ob| ob.id_as_char())
                                .collect_vec()
                        );
                        break;
                    }
                }
            }
        }
    }
}

fn iter_ordered_inclusive(a: CoordInt, b: CoordInt) -> std::ops::RangeInclusive<CoordInt> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

pub struct Solution;
impl Solver<ResultType, ResultType> for Solution {
    fn solve(&self, input: &str) -> (ResultType, ResultType) {
        solve(input)
    }
}

fn solve(input: &str) -> (ResultType, ResultType) {
    let mut tower = Tower::new_from_input(input);
    tower.drop_all_bricks();
    (0, 0)
}

#[test]
fn test_ex1() {
    let ex1 = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!((5, 0), solve(ex1));
}

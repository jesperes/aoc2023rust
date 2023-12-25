use std::fmt;

use crate::Solver;
use hashbrown::HashMap;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

type ResultType = i32;
type CoordInt = i32;
//type Coord3D = (CoordInt, CoordInt, CoordInt);
type BrickId = i32;
// type Brick = (Coord3D, Coord3D, BrickId);
// type Tower = HashMap<BrickId, Brick>;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Brick {
    corner1: Coord,
    corner2: Coord,
    id: BrickId,
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.id, self.corner1, self.corner2)
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

    fn covers_z(&self, z: CoordInt) -> bool {
        let lower_z = self.corner1.z.min(self.corner2.z);
        let upper_z = self.corner1.z.max(self.corner2.z);
        (lower_z..=upper_z).contains(&z)
    }

    fn covers_x(&self, x: CoordInt) -> bool {
        let lower_x = self.corner1.x.min(self.corner2.x);
        let upper_x = self.corner1.x.max(self.corner2.x);
        (lower_x..=upper_x).contains(&x)
    }

    fn dropn(&self, n: i32) -> Brick {
        Brick {
            id: self.id,
            corner1: self.corner1.dropn(n),
            corner2: self.corner2.dropn(n),
        }
    }

    fn overlaps(&self, other: &Brick) -> bool {
        let a = self;
        let b = other;
        overlaps_range(a.corner1.x, a.corner2.x, b.corner1.x, b.corner2.x)
            && overlaps_range(a.corner1.y, a.corner2.y, b.corner1.y, b.corner2.y)
            && overlaps_range(a.corner1.z, a.corner2.z, b.corner1.z, b.corner2.z)
    }
}

fn overlaps_range(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    let a_width = a1.max(a2) - a1.min(a2) + 1;
    let b_width = b1.max(b2) - b1.min(b2) + 1;
    let min = a1.min(a2).min(b1).min(b2);
    let max = a1.max(a2).max(b1).max(b2);
    let minmax_range = max - min + 1;
    let sum_of_range_widths = a_width + b_width;
    sum_of_range_widths > minmax_range
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashSet;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_overlaps_range() {
        let mut rng = ChaCha8Rng::seed_from_u64(2);

        let num_ranges = 1000;
        let (a, b) = (-20, 20);

        for _id in 0..num_ranges {
            let ax0 = rng.gen_range(a..b);
            let ax1 = rng.gen_range(a..b);
            let bx0 = rng.gen_range(a..b);
            let bx1 = rng.gen_range(a..b);

            let mut set1 = HashSet::new();
            let mut set2 = HashSet::new();

            for x in ax0.min(ax1)..=ax0.max(ax1) {
                set1.insert(x);
            }
            for x in bx0.min(bx1)..=bx0.max(bx1) {
                set2.insert(x);
            }

            let intersection = set1.intersection(&set2).collect_vec();
            assert_eq!(!intersection.is_empty(), overlaps_range(ax0, ax1, bx0, bx1));
        }
    }

    #[test]
    fn test_overlaps() {
        let mut bricks = Vec::new();
        let mut rng = ChaCha8Rng::seed_from_u64(2);

        let num_bricks = 100;
        let (a, b) = (0, 20);

        for id in 0..num_bricks {
            let corner1 = Coord {
                x: rng.gen_range(a..b),
                y: rng.gen_range(a..b),
                z: rng.gen_range(a..b),
            };
            let corner2 = Coord {
                x: rng.gen_range(a..b),
                y: rng.gen_range(a..b),
                z: rng.gen_range(a..b),
            };
            bricks.push(Brick {
                id,
                corner1,
                corner2,
            })
        }

        bricks
            .iter()
            .cartesian_product(bricks.iter())
            .filter(|(a, b)| a < b)
            .for_each(|(a, b)| {
                let mut set1 = HashSet::new();
                let mut set2 = HashSet::new();
                for x in a.corner1.x.min(a.corner2.x)..=a.corner1.x.max(a.corner2.x) {
                    for y in a.corner1.y.min(a.corner2.y)..=a.corner1.y.max(a.corner2.y) {
                        for z in a.corner1.z.min(a.corner2.z)..=a.corner1.z.max(a.corner2.z) {
                            set1.insert((x, y, z));
                        }
                    }
                }
                for x in b.corner1.x.min(b.corner2.x)..=b.corner1.x.max(b.corner2.x) {
                    for y in b.corner1.y.min(b.corner2.y)..=b.corner1.y.max(b.corner2.y) {
                        for z in b.corner1.z.min(b.corner2.z)..=b.corner1.z.max(b.corner2.z) {
                            set2.insert((x, y, z));
                        }
                    }
                }

                let has_intersection = set1.intersection(&set2).count() != 0;
                assert_eq!(has_intersection, a.overlaps(b));
            });
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
                .zip(1..)
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

    #[allow(dead_code)]
    fn print_tower(&self) {
        for view_z in (0..=9).rev() {
            print!("{:2} ", view_z);

            if view_z == 0 {
                println!("---");
            } else {
                for view_x in 0..=2 {
                    if let Some(brick) = self
                        .bricks
                        .values()
                        .find(|brick| brick.covers_z(view_z) && brick.covers_x(view_x))
                    {
                        print!("{}", brick.id);
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }

    /// Returns true if `brick` overlaps with any brick in the tower, false
    /// otherwise.
    fn valid_pos(&self, other: &Brick, id_to_exclude: Option<i32>) -> bool {
        if other.lowest_point() == 0 {
            // println!("... {other} is not a valid brick, it is below the bottom position");
            false
        } else if let Some(_overlaps_with) = self
            .bricks
            .values()
            .filter(|brick| {
                if let Some(id) = id_to_exclude {
                    id != brick.id
                } else {
                    true
                }
            })
            .find(|brick| brick.id != other.id && brick.overlaps(other))
        {
            // println!("... {other} is not a valid brick, it overlaps with {overlaps_with}");
            false
        } else {
            true
        }
    }

    fn drop(&self, brick: Brick, id_to_exclude: Option<i32>) -> Brick {
        let lowest_point = brick.lowest_point();
        let max_drop_dist = lowest_point;
        (1..max_drop_dist)
            .fold_while(brick, |last_valid, dz| {
                let dropped = brick.dropn(dz);
                if self.valid_pos(&dropped, id_to_exclude) {
                    // println!("... Dropped brick {dropped} is at a valid position, continuing");
                    Continue(dropped)
                } else {
                    // println!(
                    //     // "Dropping {brick} reached bottom or collides with another brick at {dropped}"
                    // );
                    Done(last_valid)
                }
            })
            .into_inner()
    }

    fn drop_all_bricks(&mut self) -> i32 {
        let mut bricks = self.bricks.values().copied().collect_vec();
        bricks.sort_by_key(|elem| elem.lowest_point());
        let mut num_dropped = 0;

        for brick in bricks {
            // println!("Dropping brick: {}", brick);
            let dropped = self.drop(brick, None);
            if dropped != brick {
                num_dropped += 1;
            }

            *self.bricks.get_mut(&dropped.id).unwrap() = dropped;
            // println!("Brick {brick} dropped to lower-most valid position {dropped}",);
        }
        num_dropped
    }

    #[allow(dead_code)]
    fn find_num_removable_bricks(&self) -> ResultType {
        let mut bricks = self.bricks.values().copied().collect_vec();
        bricks.sort_by_key(|elem| elem.lowest_point());
        let mut num_removable = 0;

        for maybe_remove_brick in &bricks {
            let mut is_removable = true;
            // println!("Checking if {maybe_remove_brick} is removable...");

            for other_brick in &bricks {
                if other_brick.id == maybe_remove_brick.id {
                    continue;
                }

                let drop = other_brick.dropn(1);
                if self.valid_pos(&drop, Some(maybe_remove_brick.id)) {
                    // println!("Brick {maybe_remove_brick} is not removable, because {other_brick} fell at least one step");
                    is_removable = false;
                    break;
                }
            }

            if is_removable {
                num_removable += 1;
            }
        }
        num_removable
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
    // tower.print_tower();
    tower.drop_all_bricks();
    // tower.print_tower();
    let p1 = tower.find_num_removable_bricks();
    (p1, 0)
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

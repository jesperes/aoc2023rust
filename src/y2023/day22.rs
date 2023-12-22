use crate::Solver;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type ResultType = i32;
type CoordInt = i32;
type Coord3D = (CoordInt, CoordInt, CoordInt);
type BrickId = i32;
// type Brick = (Coord3D, Coord3D, BrickId);
type Tower = HashMap<BrickId, Brick>;

#[derive(Debug)]
struct Brick {
    corner_a: Coord3D,
    corner_b: Coord3D,
    id: BrickId,
}

impl Brick {
    fn new(id: BrickId, corner_a: Coord3D, corner_b: Coord3D) -> Self {
        Brick {
            id,
            corner_a,
            corner_b,
        }
    }
}

pub struct Solution;
impl Solver<ResultType, ResultType> for Solution {
    fn solve(&self, input: &str) -> (ResultType, ResultType) {
        solve(input)
    }
}

fn solve(input: &str) -> (ResultType, ResultType) {
    let mut tower: Tower = HashMap::new();
    let mut bricks: Vec<Brick> = input
        .lines()
        .zip(0..)
        .map(|(line, i)| {
            let (x0, y0, z0, x1, y1, z1) = line
                .split(|c| "~,".contains(c))
                .map(|s| s.parse::<CoordInt>().unwrap())
                .collect_tuple()
                .unwrap();
            Brick::new(i, (x0, y0, z0), (x1, y1, z1))
        })
        .collect_vec();

    let (_, dropped_bricks) = drop_all_bricks(&bricks, &mut tower);

    // display_tower_xy(&tower);
    // Check which ones can be disintegrated

    // let p1 = dropped_bricks
    //     .iter()
    //     .filter(|maybe_disintegrate_brick| {
    //         let (_, _, id_to_disintegrate) = **maybe_disintegrate_brick;

    //         println!(
    //             "Checking if we can disintegrate brick {}: {:?}",
    //             id_to_char(id_to_disintegrate),
    //             maybe_disintegrate_brick
    //         );

    //         // Remove the brick
    //         let bricks0 = dropped_bricks
    //             .iter()
    //             .filter(|&(_, _, id)| *id != id_to_disintegrate)
    //             .map(|(a, b, id)| (*a, *b, *id))
    //             .collect::<Vec<_>>();
    //         // let mut tower0 = tower
    //         //     .iter()
    //         //     .filter(|&(_, id)| *id != id_to_disintegrate)
    //         //     .map(|(a, id)| (*a, *id))
    //         //     .collect::<HashMap<_, _>>();

    //         let mut tower0: Tower = HashMap::new();

    //         let (num_moved, _new_bricks) = drop_all_bricks(&bricks0, &mut tower0);

    //         let c = id_to_char(id_to_disintegrate);
    //         println!("Before disintegrating {c}");
    //         display_tower_xy(&tower);
    //         println!("After disintegrating {c}");
    //         display_tower_xy(&tower0);

    //         if num_moved == 0 {
    //             println!("When disintegrating {c}, all other bricks stayed in place");
    //             true
    //         } else {
    //             println!("When disintegrating {c}, {num_moved} other bricks moved");
    //             false
    //         }
    //     })
    //     .count();

    (p1 as i32, 0)
}

fn drop_all_bricks(bricks: &[Brick], tower: &mut Tower) -> (i32, Vec<Brick>) {
    let mut moved = 0;

    let out_bricks = bricks
        .iter()
        .map(|brick| {
            let (_, _, id) = brick;
            if let Some(dropped) = drop_one_brick(brick, tower) {
                insert_into_tower(&dropped, tower);
                println!("Brick {} moved to {:?}", id_to_char(*id), dropped);
                moved += 1;
                dropped
            } else {
                insert_into_tower(brick, tower);
                println!("Brick {} stayed at {:?}", id_to_char(*id), brick);
                *brick
            }
        })
        .collect::<Vec<_>>();
    (moved, out_bricks)
}

fn insert_into_tower(brick: &Brick, tower: &mut Tower) {
    let ((ax, ay, az), (bx, by, bz), id) = *brick;
    for x in ordered_iter(ax, bx) {
        for y in ordered_iter(ay, by) {
            for z in ordered_iter(az, bz) {
                tower.insert((x, y, z), id);
            }
        }
    }
}

fn ordered_iter(from: CoordInt, to: CoordInt) -> std::ops::RangeInclusive<i32> {
    if from < to {
        from..=to
    } else {
        to..=from
    }
}

fn drop_one_brick(brick: &Brick, tower: &Tower) -> Option<Brick> {
    let ((ax, ay, az), (bx, by, bz), id) = *brick;

    let dz = if let Some(first_tower_intersect_dz) = (1..az).find(|dz| {
        let brick0 = ((ax, ay, az - dz), (bx, by, bz - dz), id);
        intersects_with_tower(&brick0, tower)
    }) {
        let c = id_to_char(id);
        println!("While dropping {c}, it intersected with the tower after falling {first_tower_intersect_dz} steps");
        first_tower_intersect_dz - 1
    } else {
        az - 1
    };

    if dz == 0 {
        None
    } else {
        Some(((ax, ay, az - dz), (bx, by, bz - dz), id))
    }
}

/// maybe_exclude: if this is Some(id), then the specified brick is considered
/// to not be part of the tower.
fn intersects_with_tower(brick: &Brick, tower: &Tower) -> bool {
    let ((ax, ay, az), (bx, by, bz), id) = *brick;
    tower.iter().any(|((tx, ty, tz), tid)| {
        if ordered_iter(ax, bx).contains(tx)
            && ordered_iter(ay, by).contains(ty)
            && ordered_iter(az, bz).contains(tz)
        {
            let c = id_to_char(id);
            let tc = id_to_char(*tid);
            println!(
                "Tower cube {:?} (part of brick {tc}) intersects with brick {c}: {:?}",
                (tx, ty, tz),
                brick
            );
            true
        } else {
            false
        }
    })
}

fn sort_by_lowest_z_coord(bricks: &mut [Brick]) {
    bricks.sort_by(
        |((_, _, az0), (_, _, az1), _ida), ((_, _, bz0), (_, _, bz1), _idb)| {
            az0.min(az1).cmp(bz0.min(bz1))
        },
    )
}

fn id_to_char(id: i32) -> char {
    (b'A' + id as u8) as char
}
fn display_tower_xy(tower: &HashMap<Coord3D, BrickId>) {
    // display view (see example 1)
    println!("\n## Tower");
    for view_z in (0..=9).rev() {
        if view_z == 0 {
            println!("{:2} ---  ---", view_z);
        } else {
            print!("{:2} ", view_z);
            for view_x in 0..=2 {
                let v = tower
                    .iter()
                    .filter_map(|((x, _y, z), id)| {
                        if *x == view_x && *z == view_z {
                            Some(id)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>();

                let c = match v.len() {
                    0 => '.',
                    _ => id_to_char(**v.iter().next().unwrap()),
                    // => '?',
                };

                print!("{c}");
            }

            print!("  ");

            for view_y in 0..=2 {
                let v = tower
                    .iter()
                    .filter_map(|((_x, y, z), id)| {
                        if *y == view_y && *z == view_z {
                            Some(id)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>();

                let c = match v.len() {
                    0 => '.',
                    _ => id_to_char(**v.iter().next().unwrap()),
                    // _ => '?',
                };

                print!("{c}");
            }

            println!();
        }
    }
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

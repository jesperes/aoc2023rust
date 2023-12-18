use hashbrown::HashMap;
use itertools::Itertools;

use crate::Solver;
pub struct Solution;
impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        (solve_p1(input).to_string(), String::new())
    }
}

type RowCol = (i32, i32);
type RGB = (i32, i32, i32);

fn solve_p1(input: &str) -> usize {
    let mut map: HashMap<RowCol, RGB> = HashMap::new();
    map.insert((0, 0), (0, 0, 0));

    input
        .lines()
        .map(|line| {
            let (dir, len, rgb) = line
                .split(|c| c == ' ' || c == '(' || c == '#' || c == ')')
                .filter(|s| s.len() > 0)
                .next_tuple()
                .unwrap();
            let dir = dir.chars().next().unwrap();
            let len = len.parse::<i32>().unwrap();
            let r = &rgb[0..2];
            let g = &rgb[2..4];
            let b = &rgb[4..6];
            (
                dir,
                len,
                (
                    i32::from_str_radix(r, 16).unwrap(),
                    i32::from_str_radix(g, 16).unwrap(),
                    i32::from_str_radix(b, 16).unwrap(),
                ),
            )
        })
        .fold((0, 0), |pos, (dir, len, rgb)| {
            let (row_delta, col_delta) = match dir {
                'U' => (-1, 0),
                'L' => (0, -1),
                'D' => (1, 0),
                'R' => (0, 1),
                _ => unreachable!(),
            };

            (0..len).fold(pos, |p, _| {
                map.insert(p, rgb);
                let (row, col) = p;
                (row + row_delta, col + col_delta)
            })
        });

    // let max_col: i32 = *map.keys().map(|(_row, col)| col).max().unwrap();
    // let max_row: i32 = *map.keys().map(|(row, _col)| row).max().unwrap();
    // let min_col: i32 = *map.keys().map(|(_row, col)| col).min().unwrap();
    // let min_row: i32 = *map.keys().map(|(row, _col)| row).min().unwrap();

    // println!();
    // for row in min_row..=max_row {
    //     for col in min_col..=max_col {
    //         if let Some(_rgb) = map.get(&(row, col)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    flood_fill((1, 1), &mut map);
    map.len()
}

fn flood_fill(pos: RowCol, map: &mut HashMap<RowCol, RGB>) {
    let mut unfilled: Vec<RowCol> = vec![pos];

    while let Some(p) = unfilled.pop() {
        map.insert(p, (0, 0, 0));
        let (row, col) = p;

        let adjacent = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col - 1),
            (row, col + 1),
        ];

        for adj in adjacent {
            if let Some(_rgb) = map.get(&adj) {
                // already filled
            } else {
                unfilled.push(adj);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let ex = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        assert_eq!(0, solve_p1(ex));
    }
}

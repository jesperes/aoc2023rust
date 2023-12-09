extern crate test;
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use std::collections::HashMap;

pub fn solve(input: &str) -> (i64, i64) {
    let (line1, rest) = input.split_once('\n').unwrap();

    let dirs = line1.as_bytes();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in rest.trim().split('\n').collect::<Vec<&str>>() {
        let from = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(from, (left, right));
    }

    (solve_p1(&map, &dirs), solve_p2(&map, &dirs))
}

fn solve_p1(map: &HashMap<&str, (&str, &str)>, dirs: &[u8]) -> i64 {
    let mut current = "AAA";
    let mut steps = 0;

    for i in 0.. {
        let (left, right) = map.get(current).unwrap();
        let d = dirs[i as usize % dirs.len()];
        steps += 1;
        match d {
            b'L' => current = left,
            b'R' => current = right,
            _ => unreachable!(),
        }

        if current == "ZZZ" {
            break;
        }
    }

    steps
}

fn solve_p2(map: &HashMap<&str, (&str, &str)>, dirs: &[u8]) -> i64 {
    map.keys()
        .filter(|node| node.ends_with('A'))
        .into_iter()
        .map(|node| {
            let (_, count) = dirs
                .iter()
                .cycle()
                .fold_while((node, 0), |(src, count), dir| {
                    let (left, right) = map.get(src).unwrap();
                    let dest = match dir {
                        b'L' => left,
                        b'R' => right,
                        _ => unreachable!(),
                    };
                    if !dest.ends_with('Z') {
                        Continue((dest, count + 1))
                    } else {
                        Done((dest, count + 1))
                    }
                })
                .into_inner();
            count
        })
        .fold(1, |a, b| num::integer::lcm(a, b))
}

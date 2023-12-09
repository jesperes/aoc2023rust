#![feature(test)]

use glob::glob;
use lazy_regex::regex_captures;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day08;
mod day09;

#[derive(Serialize, Deserialize, Debug)]
struct Solution {
    part1: String,
    part2: String,
}

fn main() {
    let solutions: HashMap<i32, Solution> =
        serde_json::from_str(include_str!("../solutions.json")).unwrap();

    glob("inputs/input*.txt").unwrap().for_each(|e| match e {
        Ok(file) => {
            let filename = file.file_name().unwrap().to_str().unwrap();
            let (_, day) = regex_captures!(r"input(\d+).txt", filename).unwrap();
            let daynum = day.parse::<i32>().unwrap();
            let sol = solutions.get(&daynum).unwrap();
            let input = fs::read_to_string(&file).unwrap();
            run_puzzle(daynum, &input, sol);
        }
        _ => unreachable!(),
    });
}

fn run_puzzle(day: i32, input: &str, sol: &Solution) {
    let result = match day {
        1 => do_run_puzzle(day, input, sol, &day01::solve),
        2 => do_run_puzzle(day, input, sol, &day02::solve),
        3 => do_run_puzzle(day, input, sol, &day03::solve),
        4 => do_run_puzzle(day, input, sol, &day04::solve),
        _ => Err(format!("Day {day} not implemented yet")),
    };

    match result {
        Ok((p1, p2)) => println!("Day {day}: {:?} {:?}", p1, p2),
        Err(reason) => println!("{}", reason),
    }
}

// See https://github.com/rust-lang/rust/issues/43262 for an explanation of why
// the <T1 as std::str::FromStr>::Err: std::fmt::Debug thing is necessary.
fn do_run_puzzle<T1, T2>(
    day: i32,
    input: &str,
    sol: &Solution,
    f: &dyn Fn(&str) -> (T1, T2),
) -> Result<(T1, T2), String>
where
    T1: std::fmt::Debug + PartialEq + std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Debug,
    T2: std::fmt::Debug + PartialEq + FromStr,
    <T2 as std::str::FromStr>::Err: std::fmt::Debug,
{
    let actual = f(input);
    let expected = (
        sol.part1.parse::<T1>().unwrap(),
        sol.part2.parse::<T2>().unwrap(),
    );

    if actual != expected {
        return Err(format!(
            "Wrong answer for {day} part 1, expected {:?}, got {:?}",
            expected, actual
        ));
    } else {
        return Ok(actual);
    }
}

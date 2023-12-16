#![feature(int_roundings)]

use std::{
    convert::identity,
    time::{Duration, Instant},
};

use indicatif::{ProgressIterator, ProgressStyle};

use chrono::{Local, TimeZone};
use chrono_tz::US::Eastern;
use clap::Parser;
use hashbrown::HashMap;
extern crate lazy_static;

mod aoc_fetcher;
mod table;
mod utils;
// mod y2015;
// mod y2016;
// mod y2017;
// mod y2018;
// mod y2019;
// mod y2020;
// mod y2021;
// mod y2022;
mod y2023;

// Types
pub type Year = u32;
pub type Day = u32;

pub type YearSolvers = HashMap<Day, dyn Solver>;
pub type Years = HashMap<Year, YearSolvers>;

// TODO convert this so that we can benchmark without converting to/from
// strings
pub trait Solver {
    fn solve(&self, input: &String) -> (String, String);
}

// Structs

#[derive(Debug)]
pub struct PuzzleResult {
    #[allow(dead_code)]
    year: Year,
    day: Day,
    time: Duration,
    iters: u32,
    actual: (String, String),
    correct: (Option<String>, Option<String>),
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(
        short = 'y',
        long,
        num_args(1..),
        help = "Run puzzles from the specified year(s). Defaults to current year."
    )]
    year: Option<Vec<u32>>,

    #[arg(
        short = 'd',
        long,
        num_args(1..),
        help = "Run a single puzzle. If not specified, runs all puzzles for the specified year."
    )]
    days: Option<Vec<u32>>,

    #[arg(
        short = 'b',
        long,
        default_value_t = false,
        help = "Benchmark the puzzles."
    )]
    benchmark: bool,

    #[arg(
        long,
        default_value_t = 100,
        help = "Maximum number of iteration (when benchmarking)"
    )]
    max_iter: u32,

    #[arg(
        long,
        default_value_t = 50,
        help = "Maximum number of msecs/puzzle to run (when benchmarking)"
    )]
    max_msecs: u32,

    #[arg(long, default_value_t = false, help = "Sort by time")]
    sort: bool,
}

fn main() {
    let args = Cli::parse();
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let mut results = get_puzzles(&args)
        .iter()
        .progress_with_style(style)
        .map(|(y, d)| {
            if let Some(solver) = lookup_solver(*y, *d) {
                Some(invoke_solver(
                    *y,
                    *d,
                    &aoc_fetcher::maybe_fetch_puzzle_data(*y, *d),
                    &args,
                    aoc_fetcher::maybe_fetch_puzzle_solutions(*y, *d),
                    solver,
                ))
            } else {
                println!("No solver defined for ({y}, {d})");
                None
            }
        })
        .filter_map(identity)
        .collect::<Vec<_>>();

    let table = table::make_table(&mut results, &args);
    println!("{table}");
}

fn get_puzzles(args: &Cli) -> Vec<(Year, Day)> {
    if let Some(years) = &args.year {
        years
            .iter()
            .flat_map(|y| get_puzzles_for_year(*y, args))
            .collect()
    } else {
        get_puzzles_for_year(utils::current_year(), args)
    }
}

fn get_puzzles_for_year(year: Year, args: &Cli) -> Vec<(Year, Day)> {
    if let Some(days) = &args.days {
        days.iter().map(|d| (year, *d)).collect()
    } else {
        (1..=25)
            .filter(|d| is_released(year, *d))
            .map(|d| (year, d))
            .collect()
    }
}

// Return true if the given puzzle has been released
fn is_released(year: Year, day: Day) -> bool {
    Local::now()
        > Eastern
            .with_ymd_and_hms(year as i32, 12, day, 0, 0, 0)
            .unwrap()
}

fn invoke_solver(
    year: Year,
    day: Day,
    input: &String,
    args: &Cli,
    correct: (Option<String>, Option<String>),
    solver: &dyn Solver,
) -> PuzzleResult {
    let mut actual = (String::new(), String::new());
    let elapsed: Duration;
    let mut iters: u32 = 0;
    let start = Instant::now();

    if args.benchmark {
        for _ in 0..args.max_iter {
            actual = solver.solve(input);
            iters += 1;
            if start.elapsed().as_millis() > args.max_msecs as u128 {
                break;
            }
        }
        elapsed = start.elapsed();
    } else {
        let start = Instant::now();
        actual = solver.solve(input);
        elapsed = start.elapsed();
        iters = 1;
    }

    PuzzleResult {
        year,
        day,
        time: elapsed / iters,
        iters,
        actual,
        correct,
    }
}

fn lookup_solver(year: Year, day: Day) -> Option<&'static dyn Solver> {
    match (year, day) {
        (2023, 1) => Some(&y2023::day01::Solution),
        (2023, 2) => Some(&y2023::day02::Solution),
        (2023, 3) => Some(&y2023::day03::Solution),
        (2023, 4) => Some(&y2023::day04::Solution),
        (2023, 5) => Some(&y2023::day05::Solution),
        (2023, 6) => Some(&y2023::day06::Solution),
        (2023, 7) => Some(&y2023::day07::Solution),
        (2023, 8) => Some(&y2023::day08::Solution),
        (2023, 9) => Some(&y2023::day09::Solution),
        (2023, 10) => Some(&y2023::day10::Solution),
        (2023, 11) => Some(&y2023::day11::Solution),
        (2023, 12) => Some(&y2023::day12::Solution),
        (2023, 13) => Some(&y2023::day13::Solution),
        (2023, 14) => Some(&y2023::day14::Solution),
        (2023, 15) => Some(&y2023::day15::Solution),
        (2023, 16) => Some(&y2023::day16::Solution),
        // (2023, 17) => Some(&y2023::day17::Solution),
        // (2023, 18) => Some(&y2023::day18::Solution),
        // (2023, 19) => Some(&y2023::day19::Solution),
        // (2023, 20) => Some(&y2023::day20::Solution),
        // (2023, 21) => Some(&y2023::day21::Solution),
        // (2023, 22) => Some(&y2023::day22::Solution),
        // (2023, 23) => Some(&y2023::day23::Solution),
        // (2023, 24) => Some(&y2023::day24::Solution),
        // (2023, 25) => Some(&y2023::day25::Solution),
        _ => None,
    }
}

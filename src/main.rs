#![feature(int_roundings)]
use chrono::{Local, TimeZone};
use chrono_tz::US::Eastern;
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};
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

#[derive(Debug)]
enum SolverResult {
    Ok(String),
    Incorrect(String, String), // actual, result
    Unknown(String),           // puzzle result is still unknown
}

pub trait Solver<T1, T2> {
    fn solve(&self, input: &str) -> (T1, T2);
}

// Structs

#[derive(Debug)]
pub struct PuzzleResult {
    time: Duration,
    iters: u32,
    results: (SolverResult, SolverResult),
}

#[derive(Debug)]
struct PuzzleInfo {
    year: Year,
    day: Day,
    input: String,
    expected: (Option<String>, Option<String>),
}

#[derive(Debug)]
struct PuzzleRun {
    info: PuzzleInfo,
    result: PuzzleResult,
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
        default_value_t = 5000,
        help = "Maximum number of msecs/puzzle to run (when benchmarking)"
    )]
    max_msecs: u32,

    #[arg(long, default_value_t = false, help = "Sort by time")]
    sort: bool,

    #[arg(
        short = 'p',
        long,
        default_value_t = false,
        help = "Run solvers in parallel"
    )]
    parallel: bool,
}

fn main() {
    let args = Cli::parse();
    let days = get_puzzles(&args);
    let mut results: Vec<PuzzleRun> = if args.parallel {
        let m = MultiProgress::new();
        let results = days
            .into_par_iter()
            .filter_map(|puzzle_info| {
                let spinner = m.add(
                    ProgressBar::new_spinner()
                        .with_message(format!("Year {} day {}", puzzle_info.year, puzzle_info.day)),
                );
                let result = run_one_puzzle_with_progress(&puzzle_info, &args);
                spinner.finish_and_clear();
                result.map(|result| PuzzleRun {
                    info: puzzle_info,
                    result,
                })
            })
            .collect::<Vec<_>>();
        m.clear().unwrap();
        results
    } else {
        let pb = ProgressBar::new_spinner();
        let results = days
            .into_iter()
            .filter_map(|puzzle_info| {
                run_one_puzzle_with_progress(&puzzle_info, &args).map(|result| PuzzleRun {
                    info: puzzle_info,
                    result,
                })
            })
            .collect::<Vec<_>>();
        pb.finish_and_clear();
        results
    };

    let table = table::make_table(&mut results, &args);
    println!("{table}");
}

fn get_puzzles(args: &Cli) -> Vec<PuzzleInfo> {
    if let Some(years) = &args.year {
        years
            .iter()
            .flat_map(|y| get_puzzles_for_year(*y, args))
            .collect()
    } else {
        get_puzzles_for_year(utils::current_year(), args)
    }
}

fn get_puzzles_for_year(year: Year, args: &Cli) -> Vec<PuzzleInfo> {
    if let Some(days) = &args.days {
        days.iter()
            .map(|d| PuzzleInfo {
                year,
                day: *d,
                input: aoc_fetcher::maybe_fetch_puzzle_data(year, *d),
                expected: aoc_fetcher::maybe_fetch_puzzle_solutions(year, *d),
            })
            .collect()
    } else {
        (1..=25)
            .filter(|d| is_released(year, *d))
            .map(|d| PuzzleInfo {
                year,
                day: d,
                input: aoc_fetcher::maybe_fetch_puzzle_data(year, d),
                expected: aoc_fetcher::maybe_fetch_puzzle_solutions(year, d),
            })
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

fn run_one_puzzle_with_progress(puzzle_info: &PuzzleInfo, args: &Cli) -> Option<PuzzleResult> {
    match (puzzle_info.year, puzzle_info.day) {
        (2023, 1) => Some(run_with_types(puzzle_info, args, &y2023::day01::Solution)),
        (2023, 2) => Some(run_with_types(puzzle_info, args, &y2023::day02::Solution)),
        (2023, 3) => Some(run_with_types(puzzle_info, args, &y2023::day03::Solution)),
        (2023, 4) => Some(run_with_types(puzzle_info, args, &y2023::day04::Solution)),
        (2023, 5) => Some(run_with_types(puzzle_info, args, &y2023::day05::Solution)),
        (2023, 6) => Some(run_with_types(puzzle_info, args, &y2023::day06::Solution)),
        (2023, 7) => Some(run_with_types(puzzle_info, args, &y2023::day07::Solution)),
        (2023, 8) => Some(run_with_types(puzzle_info, args, &y2023::day08::Solution)),
        (2023, 9) => Some(run_with_types(puzzle_info, args, &y2023::day09::Solution)),
        (2023, 10) => Some(run_with_types(puzzle_info, args, &y2023::day10::Solution)),
        (2023, 11) => Some(run_with_types(puzzle_info, args, &y2023::day11::Solution)),
        (2023, 12) => Some(run_with_types(puzzle_info, args, &y2023::day12::Solution)),
        (2023, 13) => Some(run_with_types(puzzle_info, args, &y2023::day13::Solution)),
        (2023, 14) => Some(run_with_types(puzzle_info, args, &y2023::day14::Solution)),
        (2023, 15) => Some(run_with_types(puzzle_info, args, &y2023::day15::Solution)),
        (2023, 16) => Some(run_with_types(puzzle_info, args, &y2023::day16::Solution)),
        (2023, 17) => Some(run_with_types(puzzle_info, args, &y2023::day17::Solution)),
        (2023, 18) => Some(run_with_types(puzzle_info, args, &y2023::day18::Solution)),
        _ => None,
    }
}

fn run_with_types<T1, T2>(
    puzzle_info: &PuzzleInfo,
    args: &Cli,
    sol2: &dyn Solver<T1, T2>,
) -> PuzzleResult
where
    T1: Display + Default,
    T2: Display + Default,
{
    let actual: ((T1, T2), Duration, u32) = if args.benchmark {
        let t0 = Instant::now();
        (0..args.max_iter).fold(
            ((T1::default(), T2::default()), Duration::ZERO, 0),
            |acc, _iter| {
                let t = Instant::now();
                // Short-circuit remaining iterations if we have exceeded the time limit.
                if t0.elapsed().as_millis() > args.max_msecs as u128 {
                    acc
                } else {
                    let (_, dur, iters) = acc;
                    let actual: (T1, T2) = sol2.solve(puzzle_info.input.as_str());
                    (actual, dur + t.elapsed(), iters + 1)
                }
            },
        )
    } else {
        let t = Instant::now();
        let actual = sol2.solve(puzzle_info.input.as_str());
        (actual, t.elapsed(), 1)
    };

    let ((actual_p1, actual_p2), duration, iters) = actual;

    let (exp1, exp2) = &puzzle_info.expected;
    let results = (
        check_result(actual_p1, exp1.clone()),
        check_result(actual_p2, exp2.clone()),
    );

    PuzzleResult {
        time: duration / iters,
        iters,
        results,
    }
}

fn check_result<T>(actual: T, expected: Option<String>) -> SolverResult
where
    T: Display,
{
    let actual_str = actual.to_string();
    if let Some(expected) = expected {
        if actual_str == *expected {
            SolverResult::Ok(actual_str)
        } else {
            SolverResult::Incorrect(actual_str, expected)
        }
    } else {
        SolverResult::Unknown(actual_str)
    }
}

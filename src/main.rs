#![feature(int_roundings)]

use std::time::Duration;

use aoc_fetcher::maybe_fetch_puzzle_data;
use chrono::{Datelike, Local};
use clap::Parser;
use hashbrown::HashMap;
extern crate lazy_static;

mod aoc_fetcher;
mod y2023;

// Types
pub type Year = u32;
pub type Day = u32;

pub type YearSolvers = HashMap<Day, dyn Solver>;
pub type Years = HashMap<Year, YearSolvers>;

pub trait Solver {
    fn solve(&self, input: &String) -> (String, String);
}

// Structs
struct Result {
    year: Year,
    day: Day,
    time: Duration,
    iters: i32,
    actual: (String, String),
    expected: (String, String),
}

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'y',
        long,
        help = "Run puzzles from the specified year. Defaults to current year."
    )]
    year: Option<u32>,

    #[arg(
        short = 'd',
        long,
        help = "Run a single puzzle. If not specified, runs all puzzles for the specified year."
    )]
    day: Option<u32>,

    #[arg(
        short = 'b',
        long,
        default_value_t = false,
        help = "Benchmark the puzzles."
    )]
    benchmark: bool,
}

// fn run_puzzle<T1, T2>(
//     day: i32,
//     input: &str,
//     sol: &(Option<String>, Option<String>),
//     args: &Cli,
// ) -> Vec<Result<T1, T2>> {
//     match day {
//         1 => do_run_puzzle(day, input, args, &day01::solve),
//         2 => do_run_puzzle(day, input, args, &day02::solve),
//         3 => do_run_puzzle(day, input, args, &day03::solve),
//         4 => do_run_puzzle(day, input, args, &day04::solve),
//         5 => do_run_puzzle(day, input, args, &day05::solve),
//         6 => do_run_puzzle(day, input, args, &day06::solve),
//         7 => do_run_puzzle(day, input, args, &day07::solve),
//         8 => do_run_puzzle(day, input, args, &day08::solve),
//         9 => do_run_puzzle(day, input, args, &day09::solve),
//         10 => do_run_puzzle(day, input, args, &day10::solve),
//         11 => do_run_puzzle(day, input, args, &day11::solve),
//         12 => do_run_puzzle(day, input, args, &day12::solve),
//         13 => do_run_puzzle(day, input, args, &day13::solve),
//         14 => do_run_puzzle(day, input, args, &day14::solve),
//         15 => do_run_puzzle(day, input, args, &day15::solve),
//         _ => {
//             unreachable!()
//         }
//     }
// }

fn main() {
    let args = Cli::parse();
    // let results = get_puzzles(&args)
    //     .iter()
    //     .map(invoke_solver)
    //     .collect::<Vec<_>>();
    let d = lookup_solver(2023, 2);
    let input = aoc_fetcher::maybe_fetch_puzzle_data(2023, 2);
    if let Some(solver) = d {
        println!("{:?}", solver.solve(&input));
    }
}

fn get_puzzles(args: &Cli) -> Vec<(Year, Day)> {
    if let Some(year) = args.year {
        get_puzzles_for_year(year, args)
    } else {
        let local = Local::now();
        let date = local.date_naive();
        let (_, this_year) = date.year_ce();
        (2015..this_year)
            .flat_map(|y| get_puzzles_for_year(y, args))
            .collect::<Vec<_>>()
    }
}

fn get_puzzles_for_year(y: Year, args: &Cli) -> Vec<(Year, Day)> {
    vec![]
}

fn lookup_solver(year: Year, day: Day) -> Option<&'static dyn Solver> {
    match (year, day) {
        (2023, 1) => Some(&y2023::day01::Solution),
        (2023, 2) => Some(&y2023::day02::Solution),
        _ => None,
    }
}

// fn invoke_solver() -> Option<(String, String)> {
//     let fun =
// }

// fn run_puzzles(args: &Cli, solvers: &Solvers) {
//     // let local = Local::now();
//     // let date = local.date_naive();
//     // let (_, this_year) = date.year_ce();
//     // let year = args.year.unwrap_or(this_year);

//     // let results = args
//     //     .day
//     //     .map_or(1..=25, |d| RangeInclusive::new(d, d))
//     //     .map(|day| {
//     //         if year < this_year
//     //             || local
//     //                 > Eastern
//     //                     .with_ymd_and_hms(year as i32, 12, day, 0, 0, 0)
//     //                     .unwrap()
//     //         {
//     //             let y = years.get(&year).unwrap();

//     //         }
//     //     });
// }

// See https://github.com/rust-lang/rust/issues/43262 for an explanation of why
// the <T1 as std::str::FromStr>::Err: std::fmt::Debug thing is necessary.
// fn do_run_puzzle(
//     day: i32,
//     input: &str,
//     args: &Cli,
//     f: &dyn Fn(&str) -> (String, String),
// ) -> Result {
//     let (time, iters, (p1, p2)) = benchmark::<_>(&(|| f(input)), args);
//     Result {
//         day,
//         iters,
//         time,
//         solution: (p1.to_string(), p2.to_string()),
//     }
// }

// fn gen_sol_text<T>(maybe_expected_str: &Option<String>, actual: &T) -> String
// where
//     T: std::fmt::Debug + PartialEq + std::str::FromStr,
//     <T as std::str::FromStr>::Err: std::fmt::Debug,
// {
//     if let Some(expected_str) = maybe_expected_str {
//         let expected = expected_str.parse::<T>().unwrap();
//         if expected == *actual {
//             format!("\u{2714} {}", expected_str.green())
//         } else {
//             // This means that you have solve the puzzle correctly once, but this
//             // solution is not correct.
//             format!(
//                 "\u{2718} expected {}, got {:?}",
//                 expected_str.red().bold(),
//                 actual
//             )
//         }
//     } else {
//         // This means that we were unable to find any submitted solution in
//         // the puzzle description text.
//         format!("\u{2753} {:?}", actual)
//     }
//     .to_string()
// }

// fn benchmark<T>(f: &dyn Fn() -> T, args: &Cli) -> (Duration, i32, T) {
//     let start = std::time::Instant::now();
//     if args.benchmark {
//         let max_iter = 100;
//         let max_ms = 5;
//         let mut result;
//         let mut iters = 0;

//         loop {
//             result = f();
//             iters += 1;
//             if iters >= max_iter || start.elapsed().as_millis() > max_ms {
//                 break;
//             }
//         }

//         let elapsed_per_iter = (start.elapsed().as_nanos() / iters) as u64;
//         let duration_per_iter = Duration::from_nanos(elapsed_per_iter);
//         let result_str
//         (duration_per_iter, iters as i32, result)
//     } else {
//         // If we are not benchmarking, just execute the function once
//         let result = f();
//         (start.elapsed(), 1, result)
//     }
// }

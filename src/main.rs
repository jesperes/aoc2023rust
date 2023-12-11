use chrono::{prelude::*, Datelike, Local};
use chrono_tz::US::Eastern;
use colored::Colorize;
use reqwest::header;
use std::{fs, str::FromStr, time::Duration};
extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day08;
mod day09;
mod day10;
mod day11;

fn run_puzzle(day: i32, input: &str, sol: &(Option<String>, Option<String>)) {
    match day {
        1 => do_run_puzzle(day, input, sol, &day01::solve),
        2 => do_run_puzzle(day, input, sol, &day02::solve),
        3 => do_run_puzzle(day, input, sol, &day03::solve),
        4 => do_run_puzzle(day, input, sol, &day04::solve),
        5 => do_run_puzzle(day, input, sol, &day05::solve),
        8 => do_run_puzzle(day, input, sol, &day08::solve),
        9 => do_run_puzzle(day, input, sol, &day09::solve),
        10 => do_run_puzzle(day, input, sol, &day10::solve),
        11 => do_run_puzzle(day, input, sol, &day11::solve),
        _ => {
            println!("Day {day}: \u{2754}");
        }
    }
}

fn main() {
    let cookie = get_cookie();
    let local = Local::now();
    let date = local.date_naive();
    let (_, year) = date.year_ce();

    (1..25).for_each(|day| {
        if local
            > Eastern
                .with_ymd_and_hms(year as i32, 12, day, 0, 0, 0)
                .unwrap()
        {
            let input = maybe_fetch_puzzle_data(year, day, &cookie);
            let maybe_solution = maybe_fetch_puzzle_solutions(year, day, &cookie);
            run_puzzle(day as i32, &input, &maybe_solution);
        }
    });
}

fn maybe_fetch_puzzle_data(year: u32, day: u32, cookie: &String) -> String {
    let cache_dir = dirs::cache_dir()
        .unwrap()
        .join("aoc-data")
        .join(format!("{year}"));
    fs::create_dir_all(&cache_dir).ok();
    let puzzle_input_file = cache_dir.join(format!("input{day}.txt"));
    match puzzle_input_file.try_exists() {
        Ok(true) => fs::read_to_string(puzzle_input_file).unwrap(),
        Ok(false) => {
            let url = format!("https://adventofcode.com/{year}/day/{day}/input");
            let cookieheader = format!("session={cookie}");
            println!("\u{1f385} Fetching input data for {year} day {day}");
            let contents = reqwest::blocking::Client::new()
                .get(url)
                .header(header::COOKIE, cookieheader.trim())
                .send()
                .unwrap()
                .text()
                .unwrap();
            fs::write(puzzle_input_file, &contents).unwrap();
            contents
        }
        other => panic!("{:?}", other),
    }
}

fn maybe_fetch_puzzle_solutions(
    year: u32,
    day: u32,
    cookie: &String,
) -> (Option<String>, Option<String>) {
    let cache_dir = dirs::cache_dir()
        .unwrap()
        .join("aoc-data")
        .join(format!("{year}"));
    fs::create_dir_all(&cache_dir).ok();
    let puzzle_descr_file = cache_dir.join(format!("puzzle{day}.txt"));
    let contents = match puzzle_descr_file.try_exists() {
        Ok(true) => fs::read_to_string(puzzle_descr_file).unwrap(),
        Ok(false) => {
            let url = format!("https://adventofcode.com/{year}/day/{day}");
            let cookieheader = format!("session={cookie}");
            println!("\u{1f385} (Re)downloading puzzle description");
            let contents = reqwest::blocking::Client::new()
                .get(url)
                .header(header::COOKIE, cookieheader.trim())
                .send()
                .unwrap()
                .text()
                .unwrap();
            fs::write(puzzle_descr_file, &contents).unwrap();
            contents
        }

        other => panic!("{:?}", other),
    };

    let re = lazy_regex::regex!(r"Your puzzle answer was <code>([^<]+)</code>");
    let mut answers_in_text = re
        .captures_iter(&contents)
        .map(|capture| capture.get(1).unwrap().as_str());

    let p1 = answers_in_text.next().map(|s| s.to_string());
    let p2 = answers_in_text.next().map(|s| s.to_string());
    (p1, p2)
}

fn get_cookie() -> String {
    fs::read_to_string(dirs::home_dir().unwrap().join(".adventofcode.session"))
        .expect("Could not find cookiefile")
}

// See https://github.com/rust-lang/rust/issues/43262 for an explanation of why
// the <T1 as std::str::FromStr>::Err: std::fmt::Debug thing is necessary.
fn do_run_puzzle<T1, T2>(
    day: i32,
    input: &str,
    sol: &(Option<String>, Option<String>),
    f: &dyn Fn(&str) -> (T1, T2),
) where
    T1: std::fmt::Debug + PartialEq + std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Debug,
    T2: std::fmt::Debug + PartialEq + FromStr,
    <T2 as std::str::FromStr>::Err: std::fmt::Debug,
{
    let (time, actual) = benchmark::<(T1, T2)>(&(|| f(input)));
    let micros = time.as_micros();
    let (expected_p1_sol, expected_p2_sol) = sol;
    let (actual_p1_sol, actual_p2_sol) = actual;

    let p1_text = gen_sol_text("part 1", expected_p1_sol, &actual_p1_sol);
    let p2_text = gen_sol_text("part 2", expected_p2_sol, &actual_p2_sol);

    let col_day = format!("Day {day}:");
    let col_micros = format!("{micros} \u{b5}s");

    println!(
        "{:-10} {:10} {:40} {:40}",
        col_day, col_micros, p1_text, p2_text
    );
}

fn gen_sol_text<T>(p: &str, maybe_expected_str: &Option<String>, actual: &T) -> String
where
    T: std::fmt::Debug + PartialEq + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    if let Some(expected_str) = maybe_expected_str {
        let expected = expected_str.parse::<T>().unwrap();
        if expected == *actual {
            format!("\u{2705} {p} = {}", expected_str.green())
        } else {
            // This means that you have solve the puzzle correctly once, but this
            // solution is not correct.
            format!(
                "\u{274c} {p}: expected {}, got {:?}",
                expected_str.red().bold(),
                actual
            )
        }
    } else {
        // This means that we were unable to find any submitted solution in
        // the puzzle description text.
        format!("\u{2754} {p} = {:?} {}", actual, "submit?".yellow().bold())
    }
    .to_string()
}

fn benchmark<T>(f: &dyn Fn() -> T) -> (Duration, T) {
    let start = std::time::Instant::now();
    let max_iter = 100;
    let max_secs = 3;
    let mut result;
    let mut iters = 0;

    loop {
        result = f();
        iters += 1;
        if iters >= max_iter || start.elapsed().as_secs() > max_secs {
            break;
        }
    }

    let elapsed_per_iter = (start.elapsed().as_nanos() / iters) as u64;
    let duration_per_iter = Duration::from_nanos(elapsed_per_iter);
    (duration_per_iter, result)
}

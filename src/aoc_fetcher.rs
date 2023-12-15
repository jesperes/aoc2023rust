use std::fs;

use reqwest::header;

pub fn maybe_fetch_puzzle_data(year: u32, day: u32) -> String {
    let cookie = get_cookie();
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

pub fn maybe_fetch_puzzle_solutions(year: u32, day: u32) -> (Option<String>, Option<String>) {
    let cookie = get_cookie();
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

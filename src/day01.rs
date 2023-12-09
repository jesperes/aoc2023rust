type IsDigitFun = fn(&[u8], usize) -> Option<i32>;

pub fn solve(input: &str) -> (i32, i32) {
    input
        .lines()
        .map(|line| line.as_bytes())
        .filter(|s| s.len() > 0)
        .fold((0, 0), |(p1, p2), line| {
            (
                p1 + get_first_last(line, is_digit1),
                p2 + get_first_last(line, is_digit2),
            )
        })
}

fn get_first_last(line: &[u8], is_digit: IsDigitFun) -> i32 {
    let first = (0..line.len()).find_map(|i| is_digit(line, i)).unwrap();
    let last = (0..line.len())
        .rev()
        .find_map(|i| is_digit(line, i))
        .unwrap();
    first * 10 + last
}

fn is_digit1(line: &[u8], i: usize) -> Option<i32> {
    let c = line[i];
    if c >= b'0' && c <= b'9' {
        Some(c as i32 - '0' as i32)
    } else {
        None
    }
}

fn is_digit2(line: &[u8], i: usize) -> Option<i32> {
    match is_digit1(line, i) {
        digit @ Some(_) => digit,
        None => {
            let s = String::from_utf8_lossy(&line[i..(i + 5).min(line.len())]);
            if s.starts_with("one") {
                Some(1)
            } else if s.starts_with("two") {
                Some(2)
            } else if s.starts_with("three") {
                Some(3)
            } else if s.starts_with("four") {
                Some(4)
            } else if s.starts_with("five") {
                Some(5)
            } else if s.starts_with("six") {
                Some(6)
            } else if s.starts_with("seven") {
                Some(7)
            } else if s.starts_with("eight") {
                Some(8)
            } else if s.starts_with("nine") {
                Some(9)
            } else {
                None
            }
        }
    }
}

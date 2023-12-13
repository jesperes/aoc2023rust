// Represent the mirror as a vector of ints
type MirrorBits = Vec<u32>;

// Return true/false if
fn mirrors(ns: &MirrorBits, i: usize, smudges: u32) -> bool {
    (0..i)
        .rev()
        .zip(i..ns.len())
        .map(|(a, b)| (ns[a] ^ ns[b]).count_ones())
        .sum::<u32>()
        == smudges
}

fn summarize(grid: &str, flex: u32) -> usize {
    let mut rows = MirrorBits::new();
    let mut cols = MirrorBits::new();

    for line in grid.lines() {
        cols.resize(line.len(), 0);
        let mut row = 0;
        for (c, v) in line.bytes().enumerate() {
            cols[c] = (cols[c] << 1) | ((v == b'#') as u32);
            row = (row << 1) | ((v == b'#') as u32);
        }
        rows.push(row);
    }

    for c in 1..cols.len() {
        if mirrors(&cols, c, flex) {
            return c;
        }
    }

    for r in 1..rows.len() {
        if mirrors(&rows, r, flex) {
            return 100 * r;
        }
    }

    unreachable!();
}

pub fn solve(input: &str) -> (usize, usize) {
    (do_solve(input, 0), do_solve(input, 1))
}

fn do_solve(input: &str, smudges: u32) -> usize {
    input
        .split("\n\n")
        .map(|grid| summarize(grid, smudges))
        .sum()
}

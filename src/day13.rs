// Represent the mirror as a vector of ints
type MirrorBits = Vec<u32>;

// Detect reflections with exactly 'smudges' number of incorrect bits.
fn mirrors(ns: &MirrorBits, i: usize, smudges: u32) -> bool {
    (0..i)
        .rev()
        .zip(i..ns.len())
        .map(|(a, b)| (ns[a] ^ ns[b]).count_ones())
        .sum::<u32>()
        == smudges
}

fn summarize(grid: &str, smudges: u32) -> usize {
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

    (1..cols.len())
        // has vertical reflection?
        .filter(|c| mirrors(&cols, *c, smudges))
        .next()
        .or_else(|| {
            // if not, check horizontal reflection and multiply by 100
            Some(
                (1..rows.len())
                    .filter(|r| mirrors(&rows, *r, smudges))
                    .next()
                    .unwrap_or(0)
                    * 100,
            )
        })
        .unwrap()
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

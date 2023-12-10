use std::collections::{BTreeSet, HashMap, HashSet};
type RowCol = (i32, i32);

pub fn solve(input: &str) -> (i32, i32) {
    let mut map: HashMap<RowCol, char> = HashMap::new();
    let mut start: RowCol = (0, 0);
    let mut maxrow = 0;
    let mut maxcol = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            // To allow for tile paths to "sneak" in between pipes, we expand
            // the grid by a factor of 2. We can then compute the main loop
            // using only even coordinates, and do a flood fill using all
            // coordinates.
            let rowcol = (row as i32 * 2, col as i32 * 2);
            if char == 'S' {
                start = rowcol;
            }
            map.insert(rowcol, char);
            maxrow = maxrow.max(rowcol.0);
            maxcol = maxcol.max(rowcol.1);
        }
    }

    let (p1, mainloop) = solve_p1(&start, &map);

    // The +2 here is to compensate for the extra space in the grid
    let p2 = solve_p2(&(0, 0), &(maxrow + 2, maxcol + 2), &mainloop);

    (p1, p2)
}

fn solve_p1(start: &RowCol, map: &HashMap<RowCol, char>) -> (i32, HashSet<RowCol>) {
    let mut mainloop: HashSet<RowCol> = HashSet::new();
    let mut prev = *start;
    let (pipe1, _) = connecting_pipes(&start, &map);
    let mut curr = pipe1;
    let mut steps = 1;

    mainloop.insert(curr.0);
    mainloop.insert(curr.1);

    while curr.0 != *start {
        let (next1, next2) = connecting_pipes(&curr.0, &map);

        assert!(next1.0 .0 % 2 == 0);
        assert!(next2.0 .1 % 2 == 0);

        if next1.0 == prev {
            prev = curr.0;
            curr = next2;
        } else {
            prev = curr.0;
            curr = next1;
        }

        mainloop.insert(curr.0);
        mainloop.insert(curr.1);
        steps += 1;
    }

    (steps >> 1, mainloop)
}

fn solve_p2(start: &RowCol, limits: &(i32, i32), mainloop: &HashSet<RowCol>) -> i32 {
    let mut outer = HashSet::new();
    let mut edge = BTreeSet::new();
    let (maxrow, maxcol) = limits;
    edge.insert(*start);

    while let Some(node) = edge.pop_first() {
        let (row, col) = node;
        if mainloop.contains(&node)
            || (row < 0 || row > *maxrow || col < 0 || col > *maxcol)
            || outer.contains(&node)
        {
            continue;
        }

        outer.insert(node);

        edge.insert((row - 1, col));
        edge.insert((row + 1, col));
        edge.insert((row, col - 1));
        edge.insert((row, col + 1));
    }

    let mut inner = 0;
    for row in (0..=*maxrow).step_by(2) {
        for col in (0..=*maxcol).step_by(2) {
            let rowcol = (row as i32, col as i32);
            if outer.contains(&rowcol) || mainloop.contains(&rowcol) {
                continue;
            } else {
                inner += 1;
            }
        }
    }
    inner
}

fn connecting_pipes(
    rowcol: &RowCol,
    map: &HashMap<RowCol, char>,
) -> ((RowCol, RowCol), (RowCol, RowCol)) {
    let (row, col) = rowcol;
    let c = map.get(rowcol).unwrap();

    let north = (row - 2, *col);
    let east = (*row, col + 2);
    let south = (row + 2, *col);
    let west = (*row, col - 2);

    let north0 = (row - 1, *col);
    let east0 = (*row, col + 1);
    let south0 = (row + 1, *col);
    let west0 = (*row, col - 1);

    let n = map.get(&north).unwrap_or(&' ');
    let e = map.get(&east).unwrap_or(&' ');
    let s = map.get(&south).unwrap_or(&' ');
    let w = map.get(&west).unwrap_or(&' ');

    let mut pipes = Vec::new();

    //   |   |    --7  F--
    // --J   L--    |  |
    if (*n == 'F' || *n == '|' || *n == '7' || *n == 'S')
        && (*c == 'J' || *c == '|' || *c == 'L' || *c == 'S')
    {
        pipes.push((north, north0));
    }
    if (*e == '7' || *e == '-' || *e == 'J' || *e == 'S')
        && (*c == 'F' || *c == '-' || *c == 'L' || *c == 'S')
    {
        pipes.push((east, east0));
    }
    if (*s == 'J' || *s == '|' || *s == 'L' || *s == 'S')
        && (*c == 'F' || *c == '|' || *c == '7' || *c == 'S')
    {
        pipes.push((south, south0));
    }

    if (*w == 'L' || *w == '-' || *w == 'F' || *w == 'S')
        && (*c == 'J' || *c == '-' || *c == '7' || *c == 'S')
    {
        pipes.push((west, west0));
    }

    // There should be exactly 2 pipes connecting each pipe segment
    assert_eq!(2, pipes.len());

    (pipes[0], pipes[1])
}

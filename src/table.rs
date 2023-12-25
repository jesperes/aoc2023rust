use comfy_table::{presets::*, Attribute, Cell, Color, Table};

use crate::{Cli, PuzzleRun, SolverResult};

fn hdr_cell(text: &str) -> Cell {
    Cell::new(text)
        .fg(Color::Cyan)
        .add_attribute(Attribute::Bold)
}

pub fn make_table(runs: &mut Vec<PuzzleRun>, args: &Cli) -> comfy_table::Table {
    let mut table = Table::new();
    let total_time = runs.iter().map(|run| run.result.time).max().unwrap();

    table.load_preset(UTF8_FULL_CONDENSED).set_header(vec![
        hdr_cell("Day"),
        hdr_cell("Time"),
        hdr_cell("Iterations"),
        hdr_cell("Part 1"),
        hdr_cell("Part 2"),
    ]);

    if args.sort {
        runs.sort_by(|a, b| a.result.time.cmp(&b.result.time))
    }

    for run in runs {
        table.add_row(vec![
            Cell::new(run.info.day),
            Cell::new(format!("{:?}", run.result.time)).add_attribute(Attribute::Bold),
            Cell::new(run.result.iters),
            solution_cell(&run.result.results.0),
            if run.info.day == 25 {
                Cell::new("--").fg(Color::DarkGrey)
            } else {
                solution_cell(&run.result.results.1)
            },
        ]);
    }
    table.add_row(vec![
        Cell::new("Total").fg(Color::DarkBlue),
        Cell::new(format!("{:?}", total_time))
            .fg(Color::DarkBlue)
            .add_attribute(Attribute::Bold),
        Cell::new(""),
        Cell::new(""),
        Cell::new(""),
    ]);
    table
}

fn solution_cell(result: &SolverResult) -> Cell {
    match result {
        SolverResult::Ok(result) => Cell::new(result).fg(Color::Green),
        SolverResult::Incorrect(actual, expected) => {
            Cell::new(format!("got {actual}, expected {expected}"))
                .fg(Color::Red)
                .add_attribute(Attribute::Bold)
        }
        SolverResult::Unknown(actual) => Cell::new(actual).fg(Color::DarkYellow),
    }
}

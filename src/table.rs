use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::*, Attribute, Cell, Color, Table};

use crate::{Cli, PuzzleRun, SolverResult};

pub fn make_table(runs: &mut Vec<PuzzleRun>, args: &Cli) -> comfy_table::Table {
    let mut table = Table::new();
    let total_time = runs.iter().map(|run| run.result.time).max().unwrap();

    let table_fg_color = Color::Cyan;

    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Day").fg(table_fg_color),
            Cell::new("Time").fg(table_fg_color),
            Cell::new("Iterations").fg(table_fg_color),
            Cell::new("Part 1").fg(table_fg_color),
            Cell::new("Part 2").fg(table_fg_color),
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
            solution_cell(&run.result.results.1),
        ]);
    }
    table.add_row(vec![
        Cell::new("Total"),
        Cell::new(format!("{:?}", total_time)),
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
            Cell::new(format!("got {actual}, expected {expected}")).bg(Color::Red)
        }
        SolverResult::Unknown(actual) => Cell::new(actual).fg(Color::DarkYellow),
    }
}

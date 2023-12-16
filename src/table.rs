use std::time::Duration;

use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::*, Cell, Color, Table};

use crate::{Cli, PuzzleResult};

pub fn make_table(results: &mut Vec<PuzzleResult>, args: &Cli) -> comfy_table::Table {
    let mut table = Table::new();
    let total_time: Duration = results.iter().map(|res| res.time).sum();

    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Day"),
            Cell::new("Time"),
            Cell::new("Iterations"),
            Cell::new("Part 1"),
            Cell::new("Part 2"),
        ]);

    if args.sort {
        results.sort_by(|a, b| a.time.cmp(&b.time))
    }

    for result in results {
        table.add_row(vec![
            Cell::new(result.day),
            Cell::new(format!("{:?}", result.time)),
            Cell::new(result.iters),
            solution_cell(&result.actual.0, &result.correct.0),
            solution_cell(&result.actual.1, &result.correct.1),
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

fn solution_cell(actual: &String, correct: &Option<String>) -> Cell {
    if let Some(correct) = correct {
        if actual == correct {
            Cell::new(actual).fg(Color::Green)
        } else {
            Cell::new(format!("got {actual}, expected {correct}")).bg(Color::Red)
        }
    } else {
        Cell::new(actual).fg(Color::DarkYellow)
    }
}

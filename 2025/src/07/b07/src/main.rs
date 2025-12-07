use std::{collections::HashMap, io::BufRead};

fn main() {
    let mut grid = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            grid.push(line.into_bytes());
        }
    }

    let start_col = grid[0]
        .iter()
        .enumerate()
        .find(|(_, cell)| **cell == b'S')
        .unwrap()
        .0;

    let mut column_timelines = HashMap::new();
    column_timelines.insert(start_col, 1_i64);
    for r in 1..grid.len() {
        let mut next_column_timelines = HashMap::with_capacity(2 * column_timelines.len());
        for (&c, &num_timelines) in column_timelines.iter() {
            if grid[r][c] == b'^' {
                next_column_timelines
                    .entry(c - 1)
                    .and_modify(|n| *n += num_timelines)
                    .or_insert(num_timelines);
                next_column_timelines
                    .entry(c + 1)
                    .and_modify(|n| *n += num_timelines)
                    .or_insert(num_timelines);
            } else {
                next_column_timelines
                    .entry(c)
                    .and_modify(|n| *n += num_timelines)
                    .or_insert(num_timelines);
            }
        }
        std::mem::swap(&mut column_timelines, &mut next_column_timelines);
    }

    let mut total_timelines = 0;
    for (_, &num_timelines) in column_timelines.iter() {
        total_timelines += num_timelines;
    }
    println!("{total_timelines} timelines");
}

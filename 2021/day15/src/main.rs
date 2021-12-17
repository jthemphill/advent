use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::prelude::*;

type Grid = Vec<Vec<usize>>;

fn lowest_risk(grid: &Grid) -> usize {
    let mut q = BinaryHeap::new();

    let add_q = |q: &mut BinaryHeap<Reverse<(usize, usize, usize)>>, risk, x, y| {
        q.push(Reverse((risk, x, y)))
    };

    add_q(&mut q, 0, 0, 0);

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    while let Some(Reverse((risk, x, y))) = q.pop() {
        if seen.contains(&(x, y)) {
            continue;
        }
        if y + 1 == grid.len() && x + 1 == grid[0].len() {
            return risk;
        }
        seen.insert((x, y));
        if x > 0 {
            add_q(&mut q, risk + grid[y][x - 1], x - 1, y);
        }
        if x + 1 < grid[0].len() {
            add_q(&mut q, risk + grid[y][x + 1], x + 1, y);
        }
        if y > 0 {
            add_q(&mut q, risk + grid[y - 1][x], x, y - 1);
        }
        if y + 1 < grid.len() {
            add_q(&mut q, risk + grid[y + 1][x], x, y + 1);
        }
    }
    panic!("Couldn't reach the end...");
}

fn make_big(grid: &Grid) -> Grid {
    let mut big_grid = Vec::with_capacity(grid.len() * 5);
    for y_tile in 0..5 {
        for y in 0..grid.len() {
            let mut row = Vec::with_capacity(grid[0].len() * 5);
            for x_tile in 0..5 {
                for x in 0..grid[0].len() {
                    row.push(((y_tile + x_tile + grid[y][x] - 1) % 9) + 1);
                }
            }
            big_grid.push(row);
        }
    }
    big_grid
}

fn main() {
    let grid: Grid = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    println!("Lowest risk in small grid: {}", lowest_risk(&grid));
    let big_grid = make_big(&grid);
    println!("Lowest risk in big grid: {}", lowest_risk(&big_grid));
}

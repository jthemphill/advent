use std::collections::HashSet;

fn main() {
    let mut grid = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            grid.push(line.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>());
        }
    }

    let height = grid.len();
    let width = grid[0].len();
    let perimeter = 2 * height + 2 * width - 4;

    let mut visible = HashSet::new();
    // Looking down
    for c in 1..width - 1 {
        let mut highest = grid[0][c];
        for r in 1..height - 1 {
            if highest < grid[r][c] {
                visible.insert((r, c));
                highest = grid[r][c];
            }
        }
    }

    // Looking up
    for c in 1..width - 1 {
        let mut highest = grid[height - 1][c];
        for r in (1..height - 1).rev() {
            if highest < grid[r][c] {
                visible.insert((r, c));
                highest = grid[r][c];
            }
        }
    }

    // Looking right
    for r in 1..height - 1 {
        let mut highest = grid[r][0];
        for c in 1..width - 1 {
            if highest < grid[r][c] {
                visible.insert((r, c));
                highest = grid[r][c];
            }
        }
    }

    // Looking left
    for r in 1..height - 1 {
        let mut highest = grid[r][width - 1];
        for c in (1..width - 1).rev() {
            if highest < grid[r][c] {
                visible.insert((r, c));
                highest = grid[r][c];
            }
        }
    }

    println!("{:?}", visible);
    println!(
        "{} + {} = {}",
        perimeter,
        visible.len(),
        perimeter + visible.len()
    )
}

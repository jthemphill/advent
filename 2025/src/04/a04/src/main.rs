use std::io::BufRead;

fn is_accessible(grid: &Vec<Vec<u8>>, r: usize, c: usize) -> bool {
    let mut num_rolls = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if (dr != 0 || dc != 0)
                && grid[(r as i32 + dr) as usize][(c as i32 + dc) as usize] == b'@'
            {
                num_rolls += 1
            }
        }
    }
    num_rolls < 4
}

fn main() {
    let mut grid = Vec::new();
    let mut width = 0;
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            if grid.len() == 0 {
                width = line.len() + 2;
                grid.push([b'.'].repeat(width));
            }
            let mut row = Vec::new();
            row.push(b'.');
            row.extend_from_slice(line.as_bytes());
            row.push(b'.');
            grid.push(row);
        }
    }
    grid.push([b'.'].repeat(width));

    let mut num_accessible = 0;
    for r in 1..grid.len() - 1 {
        for c in 1..grid[r].len() - 1 {
            if grid[r][c] == b'@' && is_accessible(&grid, r, c) {
                num_accessible += 1;
            }
        }
    }
    println!("Accessible: {num_accessible}");
}

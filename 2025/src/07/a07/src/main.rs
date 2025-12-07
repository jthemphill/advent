use std::io::BufRead;

fn grid_to_str(grid: &Vec<Vec<u8>>) -> String {
    let mut s_bytes = vec![];
    for row in grid {
        s_bytes.append(&mut row.clone());
        s_bytes.push(b'\n');
    }
    String::from_utf8(s_bytes).unwrap()
}

fn main() {
    let mut grid = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            grid.push(line.into_bytes());
        }
    }

    let mut num_splits = 0;
    for r in 1..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r - 1][c] == b'S' {
                grid[r][c] = b'|';
            } else if grid[r - 1][c] == b'|' {
                if grid[r][c] == b'^' {
                    num_splits += 1;
                    grid[r][c - 1] = b'|';
                    grid[r][c + 1] = b'|';
                } else {
                    grid[r][c] = b'|';
                }
            }
        }
    }

    for row in grid {
        println!("{}", String::from_utf8(row).unwrap());
    }
    println!("{num_splits} splits");
}

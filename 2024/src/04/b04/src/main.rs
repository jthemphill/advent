fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in std::io::stdin().lines() {
        grid.push(line.unwrap().bytes().collect());
    }

    let mut count = 0;
    for r in 1..grid.len() - 1 {
        for c in 1..grid[r].len() - 1 {
            if grid[r][c] != b'A' {
                continue;
            }

            if !((grid[r - 1][c - 1] == b'M' && grid[r + 1][c + 1] == b'S')
                || (grid[r - 1][c - 1] == b'S' && grid[r + 1][c + 1] == b'M'))
            {
                continue;
            }

            if !((grid[r + 1][c - 1] == b'M' && grid[r - 1][c + 1] == b'S')
                || (grid[r + 1][c - 1] == b'S' && grid[r - 1][c + 1] == b'M'))
            {
                continue;
            }

            count += 1;
        }
    }
    println!("{}", count);
}

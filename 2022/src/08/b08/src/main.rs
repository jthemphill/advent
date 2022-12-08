fn main() {
    let mut grid = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            grid.push(line.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>());
        }
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut best = None;
    let mut best_score = 0;
    for r in 0..height {
        for c in 0..width {
            let mut left = 0;
            for c2 in (0..c).rev() {
                left += 1;
                if grid[r][c2] >= grid[r][c] {
                    break;
                }
            }

            let mut right = 0;
            for c2 in c + 1..width {
                right += 1;
                if grid[r][c2] >= grid[r][c] {
                    break;
                }
            }

            let mut up = 0;
            for r2 in (0..r).rev() {
                up += 1;
                if grid[r2][c] >= grid[r][c] {
                    break;
                }
            }

            let mut down = 0;
            for r2 in r + 1..height {
                down += 1;
                if grid[r2][c] >= grid[r][c] {
                    break;
                }
            }

            let score = left * right * up * down;
            if score > best_score {
                best = Some((r, c));
                best_score = score;
            }
        }
    }

    println!("{:?} {}", best, best_score);
}

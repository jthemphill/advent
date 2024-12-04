const XMAS: &[u8] = b"XMAS";

fn all_xmas(grid: &Vec<Vec<u8>>, r: usize, c: usize) -> i32 {
    let mut count = 0;

    let get =
        |r: usize, c: usize, i: usize| r < grid.len() && c < grid.len() && grid[r][c] == XMAS[i];

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r.wrapping_add(i), c, i))
    {
        count += 1;
    }

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r.wrapping_sub(i), c, i))
    {
        count += 1;
    }

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r, c.wrapping_add(i), i))
    {
        count += 1;
    }

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r, c.wrapping_sub(i), i))
    {
        count += 1;
    }

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r.wrapping_add(i), c.wrapping_add(i), i))
    {
        count += 1;
    }

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r.wrapping_sub(i), c.wrapping_add(i), i))
    {
        count += 1;
    }

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r.wrapping_add(i), c.wrapping_sub(i), i))
    {
        count += 1;
    }

    if (0..XMAS.len())
        .into_iter()
        .all(|i| get(r.wrapping_sub(i), c.wrapping_sub(i), i))
    {
        count += 1;
    }

    count
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in std::io::stdin().lines() {
        grid.push(line.unwrap().bytes().collect());
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] != b'X' {
                continue;
            }
            count += all_xmas(&grid, r, c);
        }
    }
    println!("{}", count);
}

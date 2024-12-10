use std::collections::{HashSet, VecDeque};

fn score_trailhead(grid: &Vec<Vec<i32>>, r: usize, c: usize) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((r, c));

    let get = |r: usize, c: usize| {
        if r < grid.len() && c < grid[r].len() {
            Some(grid[r][c])
        } else {
            None
        }
    };

    let mut nines = 0;
    while let Some((r, c)) = queue.pop_front() {
        if !seen.insert((r, c)) {
            continue;
        }

        let h = grid[r][c];

        if h == 9 {
            nines += 1;
        }

        if let Some(h2) = get(r + 1, c) {
            if h2 - h == 1 {
                queue.push_back((r + 1, c));
            }
        }
        if let Some(h2) = get(r, c + 1) {
            if h2 - h == 1 {
                queue.push_back((r, c + 1));
            }
        }
        if r > 0 {
            if let Some(h2) = get(r - 1, c) {
                if h2 - h == 1 {
                    queue.push_back((r - 1, c));
                }
            }
        }
        if c > 0 {
            if let Some(h2) = get(r, c - 1) {
                if h2 - h == 1 {
                    queue.push_back((r, c - 1));
                }
            }
        }
    }

    nines
}

fn main() {
    let mut grid: Vec<Vec<i32>> = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        grid.push(line.bytes().map(|c| (c - b'0') as i32).collect());
    }

    let mut scores = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &height) in row.iter().enumerate() {
            if height == 0 {
                let score = score_trailhead(&grid, r, c);
                println!("{score}");
                scores += score;
            }
        }
    }
    println!("{scores}");
}

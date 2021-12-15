use std::io::prelude::*;

fn is_low(heightmap: &Vec<Vec<i64>>, row: usize, col: usize) -> bool {
    let point = heightmap[row][col];
    !((row > 0 && heightmap[row - 1][col] <= point)
        || (row + 1 < heightmap.len() && heightmap[row + 1][col] <= point)
        || (col > 0 && heightmap[row][col - 1] <= point)
        || (col + 1 < heightmap[row].len() && heightmap[row][col + 1] <= point))
}

fn basin_size(heightmap: &mut Vec<Vec<i64>>, row: usize, col: usize) -> usize {
    let mut size = 0;
    let mut stack = vec![(row, col)];
    while let Some((r, c)) = stack.pop() {
        if heightmap[r][c] == 9 {
            continue;
        }
        size += 1;
        heightmap[r][c] = 9;
        if r > 0 {
            stack.push((r - 1, c));
        }
        if r + 1 < heightmap.len() {
            stack.push((r + 1, c));
        }
        if c > 0 {
            stack.push((r, c - 1));
        }
        if c + 1 < heightmap[r].len() {
            stack.push((r, c + 1));
        }
    }
    size
}

fn main() {
    let mut heightmap: Vec<Vec<i64>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.map(|line| {
                line.split("")
                    .flat_map(|n| n.parse::<i64>())
                    .collect::<Vec<i64>>()
            })
        })
        .flat_map(|nums| nums)
        .collect();
    let mut lowpoints = vec![];
    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            if is_low(&heightmap, row, col) {
                lowpoints.push((row, col));
            }
        }
    }

    let mut risk = 0;
    for &(row, col) in lowpoints.iter() {
        println!("Height of {}, {} is {}", row, col, heightmap[row][col]);
        risk += 1 + heightmap[row][col];
    }
    println!("Total risk: {}", risk);

    let mut basins = vec![];
    for &(row, col) in lowpoints.iter() {
        basins.push(basin_size(&mut heightmap, row, col));
    }
    basins.sort_by(|a, b| b.cmp(a));
    println!("Basin sizes: {:?}", basins);
    println!("Basin size product: {}", basins.iter().take(3).product::<usize>());
}

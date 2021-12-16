use std::io::prelude::*;

fn main() {
    let mut grid: Vec<Vec<i8>> = std::io::stdin()
        .lock()
        .lines()
        .flat_map(|line| line)
        .map(|line| {
            line.split("")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
}

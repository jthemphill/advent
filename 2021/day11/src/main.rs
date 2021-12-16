use std::io::prelude::*;

fn main() {
    let mut grid: Vec<Vec<(i8, bool)>> = std::io::stdin()
        .lock()
        .lines()
        .flat_map(|line| line)
        .map(|line| {
            line.trim()
                .split("")
                .filter(|c| c.len() == 1)
                .map(|c| (c.parse::<i8>().unwrap(), false))
                .collect()
        })
        .collect();

    let mut num_flashes = 0;
    let mut step = 0;
    loop {
        step += 1;
        grid.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|(energy, _)| *energy += 1));

        let mut progress = true;
        while progress {
            progress = false;
            for r in 0..10 as i8 {
                for c in 0..10 as i8 {
                    let (energy, highlighted) = &mut grid[r as usize][c as usize];
                    if *energy > 9 && !*highlighted {
                        progress = true;
                        num_flashes += 1;
                        *highlighted = true;
                        *energy = 0;

                        for dr in -1..=1 {
                            for dc in -1..=1 {
                                if (dr != 0 || dc != 0)
                                    && r + dr >= 0
                                    && r + dr < 10
                                    && c + dc >= 0
                                    && c + dc < 10
                                {
                                    grid[(r + dr) as usize][(c + dc) as usize].0 += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut sync = true;
        grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|(energy, highlighted)| {
                if *highlighted {
                    progress = true;
                    *energy = 0;
                    *highlighted = false;
                } else {
                    sync = false;
                }
            })
        });
        if sync {
            println!("Synchronized after step {}.", step);
            break;
        }
        println!("Step {}", step + 1);
        for r in 0..10 {
            println!(
                "{}",
                grid[r]
                    .iter()
                    .map(|(energy, _)| energy.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
    }
    println!("Flashes: {}", num_flashes);
}

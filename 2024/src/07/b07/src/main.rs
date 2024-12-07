use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn concat(a: i64, b: i64) -> i64 {
    let mut concatenated = a;
    let mut to_multiply = b;

    while to_multiply > 0 {
        concatenated *= 10;
        to_multiply /= 10;
    }

    concatenated + b
}

fn main() {
    let mut problems: Vec<(i64, Vec<i64>)> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (target, parts) = line.split_once(':').unwrap();

        let target: i64 = target.parse().unwrap();
        let parts: Vec<i64> = parts
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        problems.push((target, parts));
    }

    let achievable_targets: Vec<i64> = problems
        .par_iter()
        .map(|(target, parts)| {
            let mut last_iteration: HashSet<i64> = HashSet::new();
            last_iteration.insert(0);
            for &num in parts.iter() {
                let mut next_iteration: HashSet<i64> = HashSet::new();
                for &result in last_iteration.iter() {
                    if result <= *target {
                        next_iteration.insert(result + num);
                        next_iteration.insert(result * num);
                        next_iteration.insert(concat(result, num));
                    }
                }

                std::mem::swap(&mut last_iteration, &mut next_iteration);
            }

            if last_iteration.contains(&target) {
                *target
            } else {
                0
            }
        })
        .collect();

    let total: i64 = achievable_targets.iter().sum();
    println!("{total}");
}

use std::collections::HashSet;

fn main() {
    let mut num_matching = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap().trim().to_owned();
        let parts: Vec<HashSet<i32>> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" | ")
            .map(|parts| {
                parts
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        let winning = &parts[0];
        let mine = &parts[1];

        num_matching.push(winning.intersection(mine).count());
    }

    let mut num_resolved = vec![0; num_matching.len()];
    let mut num_unresolved = vec![1; num_matching.len()];
    while num_unresolved.iter().any(|&x| x > 0) {
        let mut new_unresolved = vec![0; num_matching.len()];
        for (i, n) in num_unresolved.iter().enumerate() {
            for j in 1..num_matching[i] + 1 {
                new_unresolved[i + j] += n
            }
            num_resolved[i] += n;
        }
        num_unresolved = new_unresolved;
    }

    let mut total = 0;
    for (i, n) in num_resolved.iter().enumerate() {
        println!("{} instances of card {}", n, i + 1);
        total += n;
    }
    println!("Total: {}", total);
}

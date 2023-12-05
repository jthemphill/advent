use std::collections::HashSet;

fn main() {
    let mut total = 0;
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

        let num_winning = winning.intersection(mine).count() as u32;
        if num_winning == 0 {
            continue;
        }

        let score = 2_usize.pow(num_winning - 1);
        total += score;
    }
    println!("Total: {}", total);
}

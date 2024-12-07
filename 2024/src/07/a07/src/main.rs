use std::collections::HashSet;

fn main() {
    let mut total = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (target, parts) = line.split_once(':').unwrap();

        let target: i64 = target.parse().unwrap();
        let parts: Vec<i64> = parts
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut last_iteration: HashSet<i64> = HashSet::new();
        last_iteration.insert(0);
        for num in parts.iter() {
            let mut next_iteration: HashSet<i64> = HashSet::new();
            for &result in last_iteration.iter() {
                if result <= target {
                    next_iteration.insert(result + num);
                    next_iteration.insert(result * num);
                }
            }

            std::mem::swap(&mut last_iteration, &mut next_iteration);
        }

        if last_iteration.contains(&target) {
            total += target;
        }
    }

    println!("{total}");
}

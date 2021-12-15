use std::collections::HashMap;

fn triangle(x: i64) -> i64 {
    return (x * (x + 1)) / 2;
}

fn main() {
    let mut crabs: HashMap<i64, usize> = HashMap::new();
    let mut min_pos = i64::MAX;
    let mut max_pos = i64::MIN;
    for line in include_str!("../input.txt").lines() {
        for pos in line.split(',') {
            if let Ok(pos) = pos.parse::<i64>() {
                if pos < min_pos {
                    min_pos = pos;
                }
                if pos > max_pos {
                    max_pos = pos;
                }
                *crabs.entry(pos).or_insert(0) += 1;
            }
        }
    }

    println!("{}: {}", 4, triangle(4));

    let mut best = (0, i64::MAX);
    for pos in min_pos..=max_pos {
        let mut dist = 0;
        for (&crab_pos, &count) in crabs.iter() {
            dist += triangle((crab_pos - pos).abs()) * count as i64;
        }
        if dist < best.1 {
            best = (pos, dist);
        }
    }
    println!("{:?}", best);
}

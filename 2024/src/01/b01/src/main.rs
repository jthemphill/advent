use std::collections::HashMap;

fn main() {
    let mut left = HashMap::<i32, i32>::new();
    let mut right = HashMap::<i32, i32>::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut nums = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap());
        *left.entry(nums.next().unwrap()).or_default() += 1;
        *right.entry(nums.next().unwrap()).or_default() += 1;
    }

    let mut score = 0;
    for (l, num_l) in left {
        let num_r = right.get(&l).unwrap_or(&0);
        score += l * num_l * num_r;
    }

    println!("{}", score);
}

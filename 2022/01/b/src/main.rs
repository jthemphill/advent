use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut cur = 0;
    let mut best = BinaryHeap::new();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if let Ok(i) = line.parse::<i32>() {
                cur += i;
            } else {
                best.push(Reverse(cur));
                if best.len() > 3 {
                    best.pop();
                }
                cur = 0;
            }
        }
    }
    println!("{}", best.iter().map(|Reverse(x)| x).sum::<i32>());
}

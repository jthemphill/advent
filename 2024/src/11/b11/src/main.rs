use std::collections::HashMap;
use std::io::Read;

fn digit_split(stone: i64) -> Option<(i64, i64)> {
    let num_digits = stone.checked_ilog10().unwrap() + 1;
    if num_digits % 2 == 0 {
        let left = stone / (10_i64.pow(num_digits / 2));
        let right = stone % (10_i64.pow(num_digits / 2));
        Some((left, right))
    } else {
        None
    }
}

fn blink(stones: &HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut new_stones: HashMap<i64, usize> = HashMap::with_capacity(stones.len());
    for (&stone, &count) in stones {
        if stone == 0 {
            *new_stones.entry(1).or_default() += count;
        } else if let Some((left, right)) = digit_split(stone) {
            *new_stones.entry(left).or_default() += count;
            *new_stones.entry(right).or_default() += count;
        } else {
            *new_stones.entry(stone * 2024).or_default() += count;
        }
    }
    new_stones
}

fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_to_string(&mut line).unwrap();
    let mut stones: HashMap<i64, usize> = HashMap::new();
    for stone in line.split_ascii_whitespace().map(|n| n.parse().unwrap()) {
        *stones.entry(stone).or_default() += 1;
    }
    for _ in 0..75 {
        let mut new_stones = blink(&stones);
        std::mem::swap(&mut stones, &mut new_stones);
    }
    println!("{} stones", stones.values().sum::<usize>());
}

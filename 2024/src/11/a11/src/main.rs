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

fn blink(stones: &Vec<i64>) -> Vec<i64> {
    let mut new_stones: Vec<i64> = Vec::with_capacity(stones.len());
    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if let Some((left, right)) = digit_split(stone) {
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_to_string(&mut line).unwrap();
    let mut stones: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    for _ in 0..25 {
        let mut new_stones = blink(&stones);
        std::mem::swap(&mut stones, &mut new_stones);
    }
    println!("{} stones", stones.len());
}

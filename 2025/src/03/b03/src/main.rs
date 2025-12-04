use std::collections::{BinaryHeap, HashSet};
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct BatteryUsage {
    joltage: i64,
    num_remaining: usize,
    last_index: usize,
}

impl BatteryUsage {
    fn optimistic_joltage(&self) -> i64 {
        self.joltage + 10_i64.pow(self.num_remaining as u32) - 1
    }
}

impl PartialOrd for BatteryUsage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BatteryUsage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.optimistic_joltage()
            .cmp(&other.optimistic_joltage())
            .then(other.last_index.cmp(&self.last_index))
    }
}

fn max_joltage(batteries: &[i64]) -> i64 {
    let mut seen = HashSet::new();

    let mut frontier = BinaryHeap::new();
    frontier.push(BatteryUsage {
        joltage: 0,
        num_remaining: 12,
        last_index: 0,
    });
    frontier.push(BatteryUsage {
        joltage: batteries[0] * (10_i64.pow(11)),
        num_remaining: 11,
        last_index: 0,
    });

    while let Some(usage) = frontier.pop() {
        if usage.last_index + 1 == batteries.len() && usage.num_remaining == 0 {
            return usage.joltage;
        }

        if seen.get(&usage).is_some() {
            continue;
        } else {
            seen.insert(usage.clone());
        }

        for i in (usage.last_index + 1)..batteries.len() {
            if usage.num_remaining > 0
                && usage.last_index + usage.num_remaining - 1 < batteries.len()
            {
                frontier.push(BatteryUsage {
                    joltage: usage.joltage
                        + batteries[i] * 10_i64.pow(usage.num_remaining as u32 - 1),
                    num_remaining: usage.num_remaining - 1,
                    last_index: i,
                });
            }
            if usage.last_index + usage.num_remaining < batteries.len() {
                frontier.push(BatteryUsage {
                    joltage: usage.joltage,
                    num_remaining: usage.num_remaining,
                    last_index: i,
                });
            }
        }
    }
    0
}

fn parse_batteries(battery_str: &str) -> Vec<i64> {
    battery_str
        .as_bytes()
        .into_iter()
        .map(|battery| (battery - b'0') as i64)
        .collect::<Vec<_>>()
}

fn main() {
    let mut total_joltage = 0;
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            total_joltage += max_joltage(&parse_batteries(&line));
        }
    }

    println!("Total: {total_joltage}");
}

#[test]
fn first_sample_row() {
    let batteries = parse_batteries("987654321111111");
    assert_eq!(max_joltage(&batteries), 987654321111);
}

#[test]
fn second_sample_row() {
    let batteries = parse_batteries("811111111111119");
    assert_eq!(max_joltage(&batteries), 811111111119);
}

#[test]
fn third_sample_row() {
    let batteries = parse_batteries("234234234234278");
    assert_eq!(max_joltage(&batteries), 434234234278);
}

#[test]
fn fourth_sample_row() {
    let batteries = parse_batteries("818181911112111");
    assert_eq!(max_joltage(&batteries), 888911112111);
}

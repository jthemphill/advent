use std::io::BufRead;

const MAX_STACK_SIZE: usize = 12;

fn max_joltage(batteries: &[i64]) -> i64 {
    let mut stack = Vec::with_capacity(MAX_STACK_SIZE);

    for i in 0..batteries.len() {
        while let Some(last_battery) = stack.last() {
            if *last_battery >= batteries[i] || i + MAX_STACK_SIZE - stack.len() >= batteries.len()
            {
                break;
            } else {
                stack.pop();
            }
        }
        if stack.len() < MAX_STACK_SIZE {
            stack.push(batteries[i]);
        }
    }

    assert_eq!(stack.len(), MAX_STACK_SIZE);
    let mut joltage = 0;
    for battery in stack {
        joltage *= 10;
        joltage += battery;
    }
    joltage
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

use std::io::BufRead;

fn max_joltage(batteries: &[u8]) -> i32 {
    let mut best_joltage = 0;
    let mut first_battery_joltage = 0;
    for &battery in batteries {
        let battery_joltage = (battery - b'0') as i32;
        let joltage = first_battery_joltage * 10 + battery_joltage;
        if joltage > best_joltage {
            best_joltage = joltage;
        }
        if battery_joltage > first_battery_joltage {
            first_battery_joltage = battery_joltage;
        }
    }
    best_joltage
}

fn main() {
    let mut total_joltage = 0;
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            total_joltage += max_joltage(line.as_bytes());
        }
    }

    println!("Total: {total_joltage}");
}

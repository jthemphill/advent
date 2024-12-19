use std::io::Read;

fn can_make_pattern(pattern: &Vec<u8>, towels: &Vec<Vec<u8>>) -> usize {
    let mut dp = vec![0; pattern.len()];
    for i in (0..pattern.len()).rev() {
        let pattern_slice = &pattern[i..];
        for towel in towels {
            if pattern_slice.len() < towel.len() {
                // Towel too big for rest of pattern
                continue;
            }

            let matches = towel.iter().zip(pattern_slice.iter()).all(|(t, p)| t == p);
            if !matches {
                continue;
            }

            if i + towel.len() == pattern.len() {
                dp[i] += 1;
            } else {
                assert!(i + towel.len() < pattern.len());
                dp[i] += dp[i + towel.len()];
            }
        }
    }
    dp[0]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut lines = input.split('\n');

    let towels: Vec<Vec<u8>> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.as_bytes().to_vec())
        .collect();

    lines.next().unwrap();

    let mut total = 0;
    let patterns: Vec<Vec<u8>> = lines.map(|s| s.as_bytes().to_vec()).collect();
    for pattern in patterns {
        let can_make = can_make_pattern(&pattern, &towels);
        println!("{}: {}", String::from_utf8_lossy(&pattern), can_make);
        total += can_make;
    }
    println!("{total}");
}

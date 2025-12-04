use std::io::BufRead;

fn is_invalid_with_factor(n_str: &[u8], f: usize) -> bool {
    if n_str.len() % f != 0 {
        return false;
    }

    let step = n_str.len() / f;
    for i in 0..step {
        for j in ((i + step)..n_str.len()).step_by(step) {
            if n_str[i] != n_str[j] {
                return false;
            }
        }
    }
    true
}

fn is_invalid(n: i64) -> bool {
    let n_str = n.to_string().into_bytes();
    for f in 2..=n_str.len() {
        if is_invalid_with_factor(&n_str, f) {
            return true;
        }
    }
    false
}

fn main() {
    let mut invalid_sum: i64 = 0;
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            for range in line.split(',') {
                let mut nums = range.split('-');
                let start = nums.next();
                let end = nums.next();
                if let Some(start) = start
                    && let Some(end) = end
                {
                    let start = start.parse::<i64>();
                    let end = end.parse::<i64>();
                    if let Ok(start) = start
                        && let Ok(end) = end
                    {
                        println!("Checking {start}-{end}");
                        for n in start..=end {
                            if is_invalid(n) {
                                invalid_sum += n;
                                println!("Invalid ID: {n}");
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Sum of invalid numbers: {invalid_sum}");
}

#[test]
fn test_twenty_two() {
    assert!(is_invalid_with_factor("22".as_bytes(), 2));
    assert!(is_invalid(22));
}

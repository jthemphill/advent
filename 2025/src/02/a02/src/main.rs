use std::io::BufRead;

fn is_invalid(n: i64) -> bool {
    let n_str = n.to_string().into_bytes();
    if n_str.len() % 2 == 1 {
        return false;
    }

    for i in 0..n_str.len() / 2 {
        if n_str[i] != n_str[i + n_str.len() / 2] {
            return false;
        }
    }
    true
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
                        for n in start..=end {
                            if is_invalid(n) {
                                invalid_sum += n;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Sum of invalid numbers: {invalid_sum}");
}

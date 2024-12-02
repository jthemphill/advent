fn is_safe(levels: &[i32]) -> bool {
    for i in 1..levels.len() {
        match (levels[i] - levels[i - 1]).abs() {
            1..=3 => {}
            _ => return false,
        }

        // Levels must be all increasing or all decreasing
        if (levels[i] - levels[i - 1]) * (levels[1] - levels[0]) < 0 {
            return false;
        }
    }
    true
}

fn is_safe_dampened(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let corrected_levels: Vec<i32> = Vec::from_iter(
            levels[0..i]
                .iter()
                .cloned()
                .chain(levels[i + 1..].iter().cloned()),
        );
        if is_safe(&corrected_levels) {
            return true;
        }
    }

    return false;
}

fn main() {
    let mut num_safe = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let levels: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| {
                let res = x.parse::<i32>();
                match res {
                    Ok(x) => x,
                    Err(_) => panic!("Invalid input {x}"),
                }
            })
            .collect();

        if is_safe_dampened(&levels) {
            num_safe += 1;
        }
    }

    println!("{}", num_safe);
}

fn satisfies_rules(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for (left, right) in rules {
        let left_idx = update.iter().position(|x| x == left);
        if let Some(left_idx) = left_idx {
            let right_idx = update.iter().position(|x| x == right);
            if let Some(right_idx) = right_idx {
                if right_idx < left_idx {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.contains('|') {
            let mut parts = line.split('|');
            let left = parts.next().unwrap().parse().unwrap();
            let right = parts.next().unwrap().parse().unwrap();
            rules.push((left, right));
        } else if line.contains(',') {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    let mut total = 0;
    for update in updates {
        if satisfies_rules(&update, &rules) {
            total += update[update.len() / 2];
        }
    }
    println!("{total}");
}

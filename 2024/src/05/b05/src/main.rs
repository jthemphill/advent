use std::collections::{HashMap, HashSet};

fn satisfies_rules(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for (right, lefts) in rules.iter() {
        for left in lefts {
            let left_idx = update.iter().position(|x| x == left);
            let right_idx = update.iter().position(|x| x == right);
            // println!("({left} {right}) {update:?} ({left_idx:?} {right_idx:?})");
            if let Some(left_idx) = left_idx {
                if let Some(right_idx) = right_idx {
                    if right_idx < left_idx {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn toposort(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut update_set: HashSet<i32> = update.iter().cloned().collect();
    let mut sorted: Vec<i32> = Vec::with_capacity(update.len());

    fn visit(
        update_set: &mut HashSet<i32>,
        rules: &HashMap<i32, Vec<i32>>,
        sorted: &mut Vec<i32>,
        value: i32,
    ) {
        if !update_set.contains(&value) {
            return;
        }
        if let Some(deps) = rules.get(&value) {
            for dep in deps {
                visit(update_set, rules, sorted, *dep);
            }
        }
        update_set.remove(&value);
        sorted.push(value);
    }

    for &value in update {
        visit(&mut update_set, rules, &mut sorted, value);
    }

    assert!(update_set.is_empty());
    assert_eq!(sorted.len(), update.len());

    sorted
}

fn main() {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.contains('|') {
            let mut parts = line.split('|');
            let left = parts.next().unwrap().parse().unwrap();
            let right = parts.next().unwrap().parse().unwrap();
            assert_ne!(left, right);
            rules.entry(right).or_default().push(left);
        } else if line.contains(',') {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    println!("{rules:?}");

    let mut total = 0;
    for update in updates {
        if satisfies_rules(&update, &rules) {
            println!("✅ {update:?}");
            continue;
        }

        let sorted = toposort(&update, &rules);
        println!("✨ {update:?} -> {sorted:?}");
        total += sorted[sorted.len() / 2];
    }
    println!("{total}");
}

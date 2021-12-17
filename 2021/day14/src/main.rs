use std::collections::HashMap;
use std::io::prelude::*;

type Rules = HashMap<[u8; 2], u8>;
type Counts = HashMap<u8, usize>;
type PairCounts = HashMap<[u8; 2], usize>;

fn run_rules(
    mut counts: Counts,
    old_paircounts: PairCounts,
    rules: &Rules,
) -> (Counts, PairCounts) {
    let mut paircounts = PairCounts::new();
    for (pair, pair_count) in old_paircounts {
        if let Some(&insertion) = rules.get(&pair) {
            *counts.entry(insertion).or_default() += pair_count;
            *paircounts.entry([pair[0], insertion]).or_default() += pair_count;
            *paircounts.entry([insertion, pair[1]]).or_default() += pair_count;
        } else {
            *paircounts.entry(pair).or_default() += pair_count;
        }
    }
    (counts, paircounts)
}

fn make_paircounts(s: Vec<u8>) -> (Counts, PairCounts) {
    let mut counts = Counts::new();
    let mut pairs = PairCounts::new();
    *counts.entry(s[0]).or_default() += 1;
    for i in 1..s.len() {
        *counts.entry(s[i]).or_default() += 1;
        *pairs.entry([s[i - 1], s[i]]).or_default() += 1;
    }
    (counts, pairs)
}

fn main() {
    let mut initial = vec![];
    let mut read_initial = true;
    let mut rules = Rules::new();

    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            if read_initial {
                if line.is_empty() {
                    read_initial = false;
                } else {
                    initial = line.as_bytes().to_vec();
                }
            } else {
                let mut parts = line.split(" -> ");
                let pair = parts.next().unwrap().as_bytes();
                let insertion = parts.next().unwrap().as_bytes();
                rules.insert([pair[0], pair[1]], insertion[0]);
            }
        }
    }

    let (mut counts, mut paircounts) = make_paircounts(initial);
    for step in 1..=40 {
        let new = run_rules(counts, paircounts, &rules);
        counts = new.0;
        paircounts = new.1;
        println!(
            "After step {}: {:?} (len {})",
            step,
            counts,
            counts.iter().map(|(_, v)| v).sum::<usize>()
        );
    }
    let most_common = counts.iter().max_by_key(|(_, v)| *v).unwrap();
    let least_common = counts.iter().min_by_key(|(_, v)| *v).unwrap();
    println!(
        "({:?}) - ({:?}) = {}",
        most_common,
        least_common,
        most_common.1 - least_common.1
    );
}

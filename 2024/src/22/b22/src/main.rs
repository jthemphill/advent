use std::{collections::HashMap, io::Read};

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn evolve(secret: usize) -> usize {
    let secret = mix(secret, secret * 64);
    let secret = prune(secret);

    let secret = mix(secret, secret / 32);
    let secret = prune(secret);

    let secret = mix(secret, secret * 2048);
    let secret = prune(secret);

    secret
}

fn get_prices(initial_secret: usize) -> Vec<i32> {
    let mut secrets = Vec::with_capacity(2001);
    secrets.push(initial_secret);
    let mut evolved = initial_secret;
    for _ in 0..2000 {
        evolved = evolve(evolved);
        secrets.push(evolved);
    }

    secrets
        .into_iter()
        .map(|secret| (secret % 10) as i32)
        .collect()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let initial_secrets: Vec<usize> = input
        .split('\n')
        .map(|secret| secret.parse().unwrap())
        .collect();

    let mut total_price_scores: HashMap<[i32; 4], i32> = HashMap::new();
    for initial_secret in initial_secrets {
        let mut first_price_scores_for_secret: HashMap<[i32; 4], i32> = HashMap::new();
        let prices = get_prices(initial_secret);
        for i in 4..prices.len() {
            let mut price_diffs = [0; 4];
            for j in 0..4 {
                price_diffs[j] = prices[i - 3 + j] - prices[i - 4 + j];
            }
            match first_price_scores_for_secret.entry(price_diffs) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    continue;
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(prices[i]);
                }
            };
        }
        for (price_diffs, score) in first_price_scores_for_secret {
            *total_price_scores.entry(price_diffs).or_default() += score;
        }
    }

    let best_price_score = total_price_scores
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    println!("{:?}", best_price_score);
}

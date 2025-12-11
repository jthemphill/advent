use core::num;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn get_num_paths(
    cache: &mut HashMap<String, usize>,
    adjs: &HashMap<String, Vec<String>>,
    node1: &str,
) -> usize {
    if let Some(ans) = cache.get(node1) {
        return *ans;
    }
    if node1 == "out" {
        return 1;
    }

    let mut total = 0;
    for adj in adjs.get(node1).unwrap() {
        total += get_num_paths(cache, adjs, &adj);
    }
    total
}

fn main() {
    let mut adjs: HashMap<String, Vec<String>> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();
            let node1 = parts.next().unwrap().trim_end_matches(':');

            adjs.insert(
                node1.to_owned(),
                parts.into_iter().map(|s| s.to_owned()).collect(),
            );
        }
    }

    let mut cache = HashMap::new();
    let num_you_paths = get_num_paths(&mut cache, &adjs, "you");
    println!("Num paths: {num_you_paths}");
}

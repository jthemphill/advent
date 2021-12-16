use std::collections::{HashMap, HashSet};
use std::io::prelude::*;

fn num_paths(
    edges: &HashMap<String, Vec<String>>,
    from: &str,
    can_revisit: bool,
    mut seen: HashSet<String>,
) -> usize {
    if from == "end" {
        1
    } else {
        if !from.chars().all(|c| c.is_uppercase()) {
            seen.insert(from.to_owned());
        }
        let mut total = 0;
        if let Some(from_edges) = edges.get(from) {
            for dest in from_edges.iter() {
                if !seen.contains(dest) {
                    total += num_paths(edges, dest, can_revisit, seen.clone());
                } else if can_revisit && dest != "start" {
                    total += num_paths(edges, dest, false, seen.clone());
                }
            }
        }
        total
    }
}

fn main() {
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for line in std::io::stdin().lock().lines().flat_map(|line| line) {
        let mut parts = line.split('-');
        let left = parts.next().unwrap().to_owned();
        let right = parts.next().unwrap().to_owned();
        edges.entry(left.clone()).or_default().push(right.clone());
        edges.entry(right).or_default().push(left);
    }
    println!(
        "{} paths without revisiting.",
        num_paths(&edges, "start", false, HashSet::new())
    );
    println!(
        "{} paths with revisiting",
        num_paths(&edges, "start", true, HashSet::new())
    );
}

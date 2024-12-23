use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

type Edges = HashMap<String, HashSet<String>>;

fn bron_kerbosch(edges: &Edges) -> HashSet<&String> {
    let mut stack = vec![(
        HashSet::<&String>::new(),
        edges.keys().collect::<HashSet<&String>>(),
        HashSet::<&String>::new(),
    )];
    let mut maximal_cliques = vec![];
    while let Some((must_haves, mut can_haves, mut cannot_haves)) = stack.pop() {
        if can_haves.is_empty() && cannot_haves.is_empty() {
            maximal_cliques.push(must_haves);
            continue;
        }
        let can_have_nodes: Vec<&String> = can_haves.iter().cloned().collect();
        for node in can_have_nodes {
            let node_neighbors: HashSet<&String> = edges.get(node).unwrap().iter().collect();

            let mut new_must_haves: HashSet<&String> = must_haves.clone();
            new_must_haves.insert(node);

            let new_can_haves: HashSet<&String> =
                can_haves.intersection(&node_neighbors).cloned().collect();

            let new_cannot_haves: HashSet<&String> = cannot_haves
                .intersection(&node_neighbors)
                .cloned()
                .collect();

            stack.push((new_must_haves, new_can_haves, new_cannot_haves));

            can_haves.remove(&node);
            cannot_haves.insert(node);
        }
    }
    maximal_cliques
        .into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut edges: Edges = HashMap::new();
    for line in input.split('\n') {
        let mut parts = line.split('-');
        let left = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();

        edges.entry(left.clone()).or_default().insert(right.clone());
        edges.entry(right).or_default().insert(left);
    }

    let mut max_clique: Vec<String> = bron_kerbosch(&edges).into_iter().cloned().collect();
    max_clique.sort();
    println!("{}", max_clique.join(","));
}

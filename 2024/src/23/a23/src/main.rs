use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    let mut t_nodes = vec![];
    for line in input.split('\n') {
        let mut parts = line.split('-');
        let left = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();

        if left.starts_with('t') {
            t_nodes.push(left.clone());
        }

        edges.entry(left.clone()).or_default().insert(right.clone());
        edges.entry(right).or_default().insert(left);
    }

    let mut triangles = HashSet::new();
    for node1 in &t_nodes {
        let edges1 = edges.get(node1).unwrap();
        for node2 in edges1 {
            let edges2 = edges.get(node2).unwrap();
            for node3 in edges1.intersection(edges2) {
                let mut triangle = vec![node1, node2, node3];
                triangle.sort();
                triangles.insert(triangle);
            }
        }
    }
    println!("{triangles:?} {}", triangles.len());
}

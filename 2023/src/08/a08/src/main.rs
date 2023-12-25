use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn main() {
    let node_re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();

    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();
    let directions = lines[0].as_bytes();
    assert_eq!(lines[1], "");

    let mut nodes = HashMap::new();
    for line in lines.iter().skip(2) {
        if let Some(captures) = node_re.captures(line) {
            let id = captures.get(1).unwrap().as_str().to_string();
            let left = captures.get(2).unwrap().as_str().to_string();
            let right = captures.get(3).unwrap().as_str().to_string();
            nodes.insert(id.clone(), Node { id, left, right });
        } else {
            println!("Invalid: {line}");
        }
    }

    let mut node = nodes.get("AAA").unwrap();
    let mut num_steps = 0;
    while node.id != "ZZZ" {
        println!("{}", node.id);
        let dir = directions[num_steps % directions.len()];
        node = match dir {
            b'L' => nodes.get(&node.left).unwrap(),
            b'R' => nodes.get(&node.right).unwrap(),
            _ => panic!("Unexpected direction: {dir}"),
        };
        num_steps += 1;
    }

    println!("Made it in {num_steps} steps");
}

use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Node {
    id: String,
    left: String,
    right: String,
}

/**
 * Results of the Extended Euclidean algorithm.
 *
 * Contains the greatest common denominator of `a` and `b`, along with Bézout coefficients `x` and `y` such that
 *
 *     self.a * self.bezout_a + self.b * self.bezout_b = self.gcd
 *
 * https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
 */
struct ExtendedEuclidean {
    a: isize,
    b: isize,
    gcd: usize,
    bezout_a: isize,
    bezout_b: isize,
}

impl ExtendedEuclidean {
    fn new(a: isize, b: isize) -> Self {
        let (mut old_r, mut r) = (a, b);
        let (mut old_s, mut s) = (1, 0);
        let (mut old_t, mut t) = (0, 1);

        while r != 0 {
            let quotient = old_r / r;
            (old_r, r) = (r, old_r - quotient * r);
            (old_s, s) = (s, old_s - quotient * s);
            (old_t, t) = (t, old_t - quotient * t);
        }

        let (bezout_a, bezout_b) = (old_s, old_t);
        let gcd = old_r as usize;

        assert_eq!(a * bezout_a + b * bezout_b, gcd as isize);
        ExtendedEuclidean {
            a,
            b,
            gcd,
            bezout_a,
            bezout_b,
        }
    }
}

fn main() {
    let node_re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();

    let lines: Vec<String> = std::io::stdin().lines().map(|line| line.unwrap()).collect();
    let directions = lines[0].as_bytes();
    assert_eq!(lines[1], "");

    let mut node_map = HashMap::new();
    for line in lines.iter().skip(2) {
        if let Some(captures) = node_re.captures(line) {
            let id = captures.get(1).unwrap().as_str().to_string();
            let left = captures.get(2).unwrap().as_str().to_string();
            let right = captures.get(3).unwrap().as_str().to_string();
            node_map.insert(id.clone(), Node { id, left, right });
        } else {
            println!("Invalid: {line}");
        }
    }

    let starting_nodes: Vec<Node> = node_map
        .iter()
        .filter(|(id, _)| id.ends_with("A"))
        .map(|(_, node)| node)
        .cloned()
        .collect();

    let distances = starting_nodes.iter().map(|start_node| {
        let mut num_steps = 0;
        let mut node = start_node;
        while !node.id.ends_with('Z') {
            let dir_index = num_steps % directions.len();
            let dir = directions[dir_index];

            node = match dir {
                b'L' => node_map.get(&node.left).unwrap(),
                b'R' => node_map.get(&node.right).unwrap(),
                _ => panic!("Unexpected direction: {dir}"),
            };

            num_steps += 1;
        }
        num_steps
    });

    let mut distance_lcm = 1;
    for distance in distances {
        distance_lcm /= ExtendedEuclidean::new(distance_lcm as isize, distance as isize).gcd;
        distance_lcm *= distance;
    }
    println!("LCM of distances: {distance_lcm}");
}

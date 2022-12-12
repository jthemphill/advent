use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Eq, Ord, PartialEq)]
struct Node {
    dist: usize,
    pos: (usize, usize),
    path: Vec<(usize, usize)>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

fn main() {
    let mut pos = (0, 0);
    let mut fin = (0, 0);
    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();

    let mut grid = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            grid.push(Vec::from(line.as_bytes()));
        }
    }

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            match grid[r][c] {
                b'S' => {
                    frontier.push(Node {
                        dist: 0,
                        pos: (r, c),
                        path: vec![],
                    });
                    grid[r][c] = b'a';
                }
                b'E' => {
                    fin = (r, c);
                    grid[r][c] = b'z'
                }
                _ => {}
            }
        }
    }

    while let Some(node) = frontier.pop() {
        if node.pos == fin {
            println!("Finished! {:?}", node);
        }
        if seen.contains(&node.pos) {
            continue;
        }
        seen.insert(node.pos);

        let (r, c) = node.pos;
        let h = grid[r][c];

        let mut new_path = node.path;
        new_path.push(node.pos);

        if c >= 1 && grid[r][c - 1] <= h + 1 {
            frontier.push(Node {
                pos: (r, c - 1),
                dist: node.dist + 1,
                path: new_path.clone(),
            });
        }

        if c + 1 < grid[r].len() && grid[r][c + 1] <= h + 1 {
            frontier.push(Node {
                pos: (r, c + 1),
                dist: node.dist + 1,
                path: new_path.clone(),
            });
        }

        if r >= 1 && grid[r - 1][c] <= h + 1 {
            frontier.push(Node {
                pos: (r - 1, c),
                dist: node.dist + 1,
                path: new_path.clone(),
            });
        }

        if r + 1 < grid.len() && grid[r + 1][c] <= h + 1 {
            frontier.push(Node {
                pos: (r + 1, c),
                dist: node.dist + 1,
                path: new_path.clone(),
            });
        }
    }
}

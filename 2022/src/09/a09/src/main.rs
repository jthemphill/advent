use std::collections::HashSet;

fn main() {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut seen = HashSet::new();

    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut split = line.split(" ");
            let dir = split.next().unwrap();
            let n = split.next().unwrap().parse::<i32>().unwrap();
            for _ in 0..n {
                match dir {
                    "R" => {
                        head.0 += 1;
                    }
                    "L" => {
                        head.0 -= 1;
                    }
                    "U" => {
                        head.1 += 1;
                    }
                    "D" => {
                        head.1 -= 1;
                    }
                    _ => panic!("Expected direction, got {}", dir),
                };

                if head.0 == tail.0 {
                    // Vertical
                    if tail.1 + 1 < head.1 {
                        // Up
                        tail.1 += 1;
                    } else if head.1 + 1 < tail.1 {
                        // Down
                        tail.1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    // Horizontal
                    if tail.0 + 1 < head.0 {
                        // Right
                        tail.0 += 1;
                    } else if head.0 + 1 < tail.0 {
                        // Left
                        tail.0 -= 1;
                    }
                } else if (head.0 - tail.0).abs() + (head.1 - tail.1).abs() > 2 {
                    // Diagonal
                    assert_ne!(head.0, tail.0);
                    assert_ne!(head.1, tail.1);

                    if tail.0 < head.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }

                    if tail.1 < head.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1
                    }
                }
                seen.insert(tail);
            }
        }
    }
    println!("Visited {}", seen.len());
}

use std::collections::HashSet;

fn val(x: u8) -> usize {
    if b'a' <= x && x <= b'z' {
        (x - b'a' + 1) as usize
    } else if b'A' <= x && x <= b'Z' {
        (x - b'A' + 27) as usize
    } else {
        panic!("Expected letter, got {}", x as char);
    }
}

fn main() {
    let mut sum = 0;
    let mut sets = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            sets.push(
                line.as_bytes()
                    .into_iter()
                    .cloned()
                    .collect::<HashSet<u8>>(),
            );
            if sets.len() == 3 {
                let mut overlapping: HashSet<u8> =
                    sets[0].intersection(&sets[1]).cloned().collect();
                overlapping = overlapping.intersection(&sets[2]).cloned().collect();
                let overlap = overlapping.iter().next().unwrap();
                println!("{}", *overlap as char);
                sum += val(*overlap);
                sets = vec![];
            }
        }
    }
    println!("Total: {}", sum);
}

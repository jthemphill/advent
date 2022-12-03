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
    let mut set = HashSet::new();
    let mut i = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if i == 0 {
                set = line.as_bytes().into_iter().cloned().collect();
            }
            for c in line.as_bytes() {
                if !set.contains(c) {
                    set.remove(c);
                }
            }
            if i == 2 {
                assert_eq!(set.len(), 1);
                let overlap = *set.iter().next().unwrap();
                println!("{}", overlap as char);
                sum += val(overlap);
                i = 0;
            }
        }
    }
    println!("Total: {}", sum);
}

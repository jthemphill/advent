use std::collections::HashMap;

const LIMIT: usize = 100_000;

fn main() {
    let mut cwd: Vec<String> = vec![];
    let mut total_sizes: HashMap<String, usize> = HashMap::new();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut tokens = line.split(' ');
            match tokens.next() {
                Some("$") => match tokens.next() {
                    Some("cd") => match tokens.next() {
                        Some("/") => {
                            cwd = vec!["".to_string()];
                        }
                        Some("..") => {
                            cwd.pop().unwrap();
                        }
                        Some(dir) => {
                            cwd.push(dir.to_string());
                        }
                        None => panic!("Attempted to cd with no target dir"),
                    },
                    Some("ls") => {}
                    Some(o) => panic!("Unexpected command: {}", o),
                    None => panic!("No command"),
                },
                Some("dir") => {}
                Some(sz) => {
                    let sz = sz.parse::<usize>().unwrap();
                    for i in 0..cwd.len() {
                        *total_sizes.entry(cwd[0..=i].join("/")).or_default() += sz;
                    }
                }
                None => panic!("No tokens in line"),
            }
        }
    }
    let total_kept = total_sizes
        .values()
        .filter(|&&sz| sz <= LIMIT)
        .sum::<usize>();
    println!("Total: {}", total_kept);
}

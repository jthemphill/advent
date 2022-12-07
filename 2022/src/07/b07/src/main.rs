use std::collections::HashMap;

const DISK_SPACE: usize = 70_000_000;
const NEEDED_FREE_SPACE: usize = 30_000_000;

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
    let free_space = DISK_SPACE - total_sizes.get("").unwrap();
    let need_to_free = NEEDED_FREE_SPACE - free_space;
    let size_to_free = total_sizes
        .values()
        .filter(|&&sz| sz >= need_to_free)
        .min()
        .unwrap();
    println!("Free: {}", size_to_free);
}

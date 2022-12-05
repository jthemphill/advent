use regex::Regex;

enum ParseMode {
    MakingStacks,
    Running,
}

fn main() {
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];

    let mut mode = ParseMode::MakingStacks;

    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            match mode {
                ParseMode::MakingStacks => {
                    if !line.contains('[') {
                        for s in &mut stacks {
                            s.reverse();
                        }
                        mode = ParseMode::Running;
                    } else {
                        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                            while stacks.len() <= i {
                                stacks.push(vec![]);
                            }
                            if c.is_ascii_uppercase() {
                                stacks[i].push(c);
                            }
                        }
                    }
                }
                ParseMode::Running => {
                    if let Some(cap) = move_re.captures(&line) {
                        let n = cap[1].parse::<usize>().unwrap();
                        let src = cap[2].parse::<usize>().unwrap() - 1;
                        let dst = cap[3].parse::<usize>().unwrap() - 1;

                        let mut src_stack = std::mem::take(&mut stacks[src]);
                        let mut dst_stack = std::mem::take(&mut stacks[dst]);
                        dst_stack.extend(src_stack.drain((src_stack.len() - n)..));
                        stacks[src] = src_stack;
                        stacks[dst] = dst_stack;
                    }
                }
            };
        }
    }
    println!(
        "{}",
        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    );
}

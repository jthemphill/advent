use std::{
    ascii::escape_default,
    collections::{hash_map::Entry, HashMap, VecDeque},
    io::Read,
};

type Edge = (u8, u8);

const N: usize = 25;

fn get_numeric_edges(pos: u8) -> &'static [Edge] {
    match pos {
        b'A' => &[(b'<', b'0'), (b'^', b'3')],
        b'0' => &[(b'>', b'A'), (b'^', b'2')],
        b'1' => &[(b'>', b'2'), (b'^', b'4')],
        b'2' => &[(b'v', b'0'), (b'<', b'1'), (b'>', b'3'), (b'^', b'5')],
        b'3' => &[(b'v', b'A'), (b'<', b'2'), (b'^', b'6')],
        b'4' => &[(b'v', b'1'), (b'>', b'5'), (b'^', b'7')],
        b'5' => &[(b'v', b'2'), (b'<', b'4'), (b'>', b'6'), (b'^', b'8')],
        b'6' => &[(b'v', b'3'), (b'<', b'5'), (b'^', b'9')],
        b'7' => &[(b'v', b'4'), (b'>', b'8')],
        b'8' => &[(b'v', b'5'), (b'<', b'7'), (b'>', b'9')],
        b'9' => &[(b'v', b'6'), (b'<', b'8')],
        _ => panic!("Unexpected position: {}", escape_default(pos)),
    }
}

fn get_direction_edges(pos: u8) -> &'static [Edge] {
    match pos {
        b'A' => &[(b'v', b'>'), (b'<', b'^')],
        b'^' => &[(b'v', b'v'), (b'>', b'A')],
        b'<' => &[(b'>', b'v')],
        b'v' => &[(b'<', b'<'), (b'>', b'>'), (b'^', b'^')],
        b'>' => &[(b'<', b'v'), (b'^', b'A')],
        _ => panic!("Unexpected position: {}", escape_default(pos)),
    }
}

fn cost_to_push_button(
    cache: &mut HashMap<(u8, u8, usize), usize>,
    start: u8,
    target: u8,
    depth: usize,
) -> usize {
    let answer = cache.get(&(start, target, depth));
    if let Some(&answer) = answer {
        return answer;
    }

    let mut history = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((start, vec![]));

    let mut best = std::usize::MAX;
    while let Some((pos, mut path)) = queue.pop_front() {
        if pos == target {
            path.push(b'A');
            let cost = cost_of_path(cache, &path, depth + 1);
            best = best.min(cost);
            continue;
        }
        let edges = if depth == 0 {
            get_numeric_edges(pos)
        } else {
            get_direction_edges(pos)
        };
        for &(dir, next) in edges {
            let entry = history.entry(next);
            match entry {
                Entry::Occupied(cost) => {
                    if path.len() > *cost.get() {
                        continue;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(path.len());
                }
            };
            let mut next_path = path.clone();
            next_path.push(dir);
            queue.push_back((next, next_path));
        }
    }
    cache.insert((start, target, depth), best);
    best
}

fn cost_of_path(cache: &mut HashMap<(u8, u8, usize), usize>, path: &[u8], depth: usize) -> usize {
    if depth > N {
        path.len()
    } else {
        let mut pos = b'A';
        let mut total_cost = 0;
        for &target in path {
            total_cost += cost_to_push_button(cache, pos, target, depth);
            pos = target;
        }
        total_cost
    }
}

fn code_to_number(code: &[u8]) -> usize {
    let mut num = 0;

    for c in code {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c - b'0') as usize;
        }
    }

    num
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let codes: Vec<Vec<u8>> = input
        .split('\n')
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut cache = HashMap::new();
    let mut total_complexity = 0;
    for code in &codes {
        let code_cost = cost_of_path(&mut cache, code, 0);
        let num = code_to_number(code);
        let complexity = code_cost * num;
        println!("{} * {} = {}", code_cost, num, complexity);
        total_complexity += complexity;
    }
    println!("Total: {total_complexity}");
}

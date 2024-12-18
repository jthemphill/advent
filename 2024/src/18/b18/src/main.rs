use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io::Read,
};

const N: usize = 70;

fn h((x, y): (usize, usize)) -> usize {
    assert!(x <= N);
    assert!(y <= N);
    N - x + N - y
}

fn reachable(hazards: &Vec<(usize, usize)>, num_bytes: usize) -> bool {
    let corrupted: HashSet<(usize, usize)> = hazards.iter().take(num_bytes).cloned().collect();

    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();
    let start = (0, 0);
    frontier.push(Reverse((h(start), 0, start, vec![start])));

    while let Some(Reverse((_, time, pos, path))) = frontier.pop() {
        if !seen.insert(pos) {
            continue;
        }

        if pos == (N, N) {
            return true;
        }

        let (x, y) = pos;
        for (x2, y2) in [
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
        ] {
            if x2 > N || y2 > N {
                continue;
            }

            if !corrupted.contains(&(x2, y2)) {
                let mut path = path.clone();
                path.push((x2, y2));
                let state = (time + h((x2, y2)) + 1, time + 1, (x2, y2), path);
                frontier.push(Reverse(state));
            }
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut hazards: Vec<(usize, usize)> = vec![];
    for line in input.split('\n') {
        let mut coords = line.split(',');
        let x: usize = coords.next().unwrap().parse().unwrap();
        let y: usize = coords.next().unwrap().parse().unwrap();
        hazards.push((x, y));
    }

    let mut last_possible = 0;
    let mut first_impossible = hazards.len();
    while last_possible + 1 < first_impossible {
        let mid = last_possible + (first_impossible - last_possible) / 2;
        if reachable(&hazards, mid) {
            last_possible = mid;
        } else {
            first_impossible = mid;
        }
    }
    println!("{last_possible} {:?}", hazards[last_possible]);
}

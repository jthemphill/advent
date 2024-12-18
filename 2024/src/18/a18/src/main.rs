use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io::Read,
};

const N: usize = 70;
const NUM_BYTES: usize = 1024;

fn h((x, y): (usize, usize)) -> usize {
    assert!(x <= N);
    assert!(y <= N);
    N - x + N - y
}

fn render(corrupted: &HashSet<(usize, usize)>, path: &Vec<(usize, usize)>) {
    let path: HashSet<(usize, usize)> = path.into_iter().cloned().collect();
    for y in 0..=N {
        let mut line = vec![b'.'; N + 1];
        for x in 0..=N {
            if path.contains(&(x, y)) {
                line[x] = b'O';
            } else if corrupted.contains(&(x, y)) {
                line[x] = b'#';
            }
        }
        println!("{}", String::from_utf8(line).unwrap());
    }
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

    let corrupted: HashSet<(usize, usize)> = hazards.iter().take(NUM_BYTES).cloned().collect();
    assert_eq!(corrupted.len(), NUM_BYTES);

    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();
    let start = (0, 0);
    frontier.push(Reverse((h(start), 0, start, vec![start])));

    while let Some(Reverse((estimate, time, pos, path))) = frontier.pop() {
        if !seen.insert(pos) {
            continue;
        }

        if pos == (N, N) {
            println!("Made it in {time}");
            render(&corrupted, &path);
            break;
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
}

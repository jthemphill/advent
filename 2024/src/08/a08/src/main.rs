use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut input: Vec<u8> = input.into_bytes();

    let width = input
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == b'\n')
        .unwrap()
        .0;

    let height = (input.len() + 1) / (width + 1);

    let mut antennae: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for (i, &c) in input.iter().enumerate() {
        match c {
            b'\n' | b'.' => {}
            _ => {
                let x = (i % (width + 1)) as i32;
                let y = (i / (width + 1)) as i32;
                antennae.entry(c).or_default().push((x, y));
            }
        };
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, antennae) in antennae {
        for (i, &(x1, y1)) in antennae.iter().enumerate() {
            for &(x2, y2) in antennae.iter().skip(i + 1) {
                let dx = x2 - x1;
                let dy = y2 - y1;

                let mut maybe_insert = |(x, y)| {
                    if 0 <= x && x < (width as i32) && 0 <= y && y < (height as i32) {
                        antinodes.insert((x, y));
                    }
                };

                maybe_insert((x1 - dx, y1 - dy));
                maybe_insert((x2 + dx, y2 + dy));
            }
        }
    }

    for &(x, y) in antinodes.iter() {
        input[(y as usize) * (width + 1) + (x as usize)] = b'#';
    }

    println!("{}", String::from_utf8(input).unwrap());

    let num_antinodes = antinodes.len();
    println!("{num_antinodes}");
}

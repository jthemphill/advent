use std::collections::{HashMap, HashSet, VecDeque};
use std::io::prelude::*;

use rayon::prelude::*;

type Point = [i64; 3];

fn add(p1: &Point, p2: &Point) -> Point {
    [p1[0] + p2[0], p1[1] + p2[1], p1[2] + p2[2]]
}

fn subtract(p1: &Point, p2: &Point) -> Point {
    [p1[0] - p2[0], p1[1] - p2[1], p1[2] - p2[2]]
}

fn distance(p1: &Point, p2: &Point) -> i64 {
    subtract(p1, p2).into_iter().map(|dim| dim.abs()).sum()
}

// I
// 1 0 0
// 0 1 0
// 0 0 1

// R_x
// 1 0 0
// 0 0 -1
// 0 1 0

// R_y
// 0 0 1
// 0 1 0
// -1 0 0

// R_z
// 0 -1 0
// 1 0 0
// 0 0 1

/// Imagine you have a d6 with 1 facing upwards and 2 facing forwards. "Face"
/// represents the face that will be facing upwards after the rotation.
///
/// Imagine you have a d6 with 2 facing forwards. "rot" represents the face
/// that will be facing forwards after the rotation.
#[derive(Clone, Copy, Debug)]
struct Rotation {
    up: u8,
    forward: u8,
}

impl Rotation {
    fn new(up: u8, forward: u8) -> Rotation {
        if up < 1 || up > 6 {
            panic!("Invalid up: {}", up);
        }
        if forward < 2 || forward > 5 {
            panic!("Invalid forward: {}", forward)
        }
        Rotation { up, forward }
    }

    fn all() -> [Rotation; 24] {
        let mut rotations = [Rotation { up: 1, forward: 2 }; 24];
        for up in 0..6 {
            for forward in 0..4 {
                rotations[up * 4 + forward] = Rotation::new((up + 1) as u8, (forward + 2) as u8)
            }
        }
        rotations
    }

    const fn rotate(&self, mut p: Point) -> Point {
        p = match self.up {
            1 => p,
            2 => [p[2], p[1], -p[0]],
            3 => [p[0], p[2], -p[1]],
            4 => [p[0], -p[2], p[1]],
            5 => [-p[2], p[1], p[0]],
            6 => [p[0], -p[1], -p[2]],
            _ => [-999, -999, -999],
        };
        p = match self.forward {
            2 => p,
            3 => [-p[1], p[0], p[2]],
            4 => [p[1], -p[0], p[2]],
            5 => [-p[0], -p[1], p[2]],
            _ => [-999, -999, -999],
        };
        p
    }
}

/// Iff beacons1 and beacons2 share at least 12 overlapping beacons without
/// needing rotation, return a Point that represents the translation.
fn translate(beacons1: &HashSet<Point>, beacons2: &HashSet<Point>) -> Option<Point> {
    beacons1
        .iter()
        .flat_map(|beacon1| beacons2.iter().map(|beacon2| subtract(beacon1, beacon2)))
        .filter(|translate_2_to_1| {
            beacons2
                .iter()
                .filter(|beacon2| beacons1.contains(&add(beacon2, &translate_2_to_1)))
                .count()
                >= 12
        })
        .next()
}

/// Iff beacons1 and beacons2 share at least 12 overlapping beacons, return a
/// Point that represents the translation, a Rotation object representing
/// the rotation, and the number of overlapping beacons we found.
fn rotate_and_translate(
    mapped_beacons: &HashSet<Point>,
    unmapped_beacons: &HashSet<Point>,
) -> Option<(Point, Vec<Point>)> {
    Rotation::all()
        .into_par_iter()
        .flat_map_iter(|rotation| {
            let rotated_beacons = &unmapped_beacons
                .iter()
                .map(|beacon2| rotation.rotate(*beacon2))
                .collect();
            if let Some(location) = translate(mapped_beacons, rotated_beacons) {
                Some((
                    location,
                    rotated_beacons
                        .into_iter()
                        .map(|beacon| add(beacon, &location))
                        .collect(),
                ))
            } else {
                None
            }
        })
        .find_any(|_| true)
}

fn parse_input() -> Vec<HashSet<Point>> {
    let mut scanners = vec![];
    let mut scanner = HashSet::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if !scanner.is_empty() {
                scanners.push(scanner);
            }
            scanner = HashSet::new();
        } else if !line.starts_with("--- scanner") {
            let mut point: Point = [0; 3];
            let mut parts = line.split(',').map(|n| n.parse().unwrap());
            for i in 0..3 {
                point[i] = parts.next().unwrap();
            }
            scanner.insert(point);
        }
    }
    if !scanner.is_empty() {
        scanners.push(scanner);
    }
    scanners
}

fn main() {
    println!("Uh...");
    let scanners = parse_input();

    let mut scanners = scanners.into_iter().enumerate();
    let (_, mut mapped_beacons) = scanners.next().unwrap();

    let mut unmapped_scanners: VecDeque<(usize, HashSet<Point>)> = scanners.collect();
    let mut scanner_locations: HashMap<usize, Point> = HashMap::new();
    scanner_locations.insert(0, [0, 0, 0]);

    while let Some((i, unmapped_scanner)) = unmapped_scanners.pop_front() {
        println!(
            "{} unmapped remaining. Mapping scanner {}...",
            unmapped_scanners.len() + 1,
            i
        );
        if let Some((location, beacons)) = rotate_and_translate(&mapped_beacons, &unmapped_scanner)
        {
            scanner_locations.insert(i, location);
            for beacon in beacons {
                mapped_beacons.insert(beacon);
            }
        } else {
            unmapped_scanners.push_back((i, unmapped_scanner));
        }
    }

    let num_beacons: usize = mapped_beacons.len();
    println!("{} beacons.", num_beacons);
    println!("Locations: {:?}", scanner_locations);
    let max_distance = (0..scanner_locations.len())
        .flat_map(|i| {
            (i + 1..scanner_locations.len())
                .map(move |j| (i, j))
                .map(|(i, j)| distance(&scanner_locations[&i], &scanner_locations[&j]))
        })
        .max()
        .unwrap();
    println!("Maximum distance: {}", max_distance);
}

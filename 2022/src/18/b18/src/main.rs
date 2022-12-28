use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Orientation {
    XY,
    YZ,
    XZ,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Face {
    x: i64,
    y: i64,
    z: i64,
    o: Orientation,
}

impl Face {
    fn pair(coords: (i64, i64, i64), o: Orientation) -> (Self, Self) {
        let (x, y, z) = coords;
        let s1 = Self { x, y, z, o };
        let s2 = match o {
            Orientation::YZ => Self { x: x + 1, y, z, o },
            Orientation::XZ => Self { x, y: y + 1, z, o },
            Orientation::XY => Self { x, y, z: z + 1, o },
        };
        (s1, s2)
    }

    fn from_cube(coords: (i64, i64, i64)) -> Vec<Self> {
        let mut faces = vec![];
        let mut add_pair = |(f1, f2)| {
            faces.push(f1);
            faces.push(f2)
        };
        add_pair(Face::pair(coords, Orientation::XY));
        add_pair(Face::pair(coords, Orientation::YZ));
        add_pair(Face::pair(coords, Orientation::XZ));
        faces
    }
}

#[derive(Clone, Debug, Default)]
struct FaceCounter {
    map: HashMap<Face, usize>,
}

impl FaceCounter {
    fn add(&mut self, f: Face) {
        let count = self.map.entry(f).or_default();
        *count += 1;
    }

    fn extend(&mut self, fs: impl Iterator<Item=Face>) {
        for f in fs {
            self.add(f);
        }
    }
}

fn main() {
    let mut cubes = HashSet::new();
    let mut face_counts = FaceCounter::default();
    let mut min_dim = 0;
    let mut max_dim = 50;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let coords: Vec<i64> = line.split(",").map(|c| c.parse().unwrap()).collect();
            assert_eq!(coords.len(), 3);
            min_dim = min_dim.min(*coords.iter().min().unwrap() - 1);
            max_dim = max_dim.max(*coords.iter().max().unwrap() + 1);
            let coords = (coords[0], coords[1], coords[2]);
            face_counts.extend(Face::from_cube(coords).into_iter());
            cubes.insert(coords);
        }
    }

    let mut frontier = vec![(min_dim, min_dim, min_dim)];
    let mut visited = HashSet::new();
    let mut num_seen_faces = 0;
    while let Some(coords) = frontier.pop() {
        let (x, y, z) = coords;
        if [x, y, z].into_iter().any(|c| c < min_dim || c > max_dim) {
            continue;
        }
        if !visited.insert(coords) {
            continue;
        }
        if cubes.contains(&coords) {
            continue;
        }
        for face in Face::from_cube(coords) {
            if face_counts.map.get(&face) == Some(&1) {
                num_seen_faces += 1;
            }
        }
        frontier.push((x - 1, y, z));
        frontier.push((x + 1, y, z));
        frontier.push((x, y - 1, z));
        frontier.push((x, y + 1, z));
        frontier.push((x, y, z - 1));
        frontier.push((x, y, z + 1));
    }
    println!("Saw {} faces", num_seen_faces);
}

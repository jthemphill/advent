use std::collections::HashMap;

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

    fn add_pair(&mut self, fs: (Face, Face)) {
        self.add(fs.0);
        self.add(fs.1);
    }
}

fn main() {
    let mut faces = FaceCounter::default();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let coords: Vec<i64> = line.split(",").map(|c| c.parse().unwrap()).collect();
            assert_eq!(coords.len(), 3);
            let coords = (coords[0], coords[1], coords[2]);
            faces.add_pair(Face::pair(coords, Orientation::XY));
            faces.add_pair(Face::pair(coords, Orientation::YZ));
            faces.add_pair(Face::pair(coords, Orientation::XZ));
        }
    }
    println!("Exterior Faces: {}", faces.map.values().filter(|&&count| count == 1).count());
}

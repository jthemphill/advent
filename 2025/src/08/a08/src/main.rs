use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    io::BufRead,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist_sq(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

struct UnionFind {
    points: Vec<Point>,
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(points: Vec<Point>) -> Self {
        let parents = (0..points.len()).map(|i| i).collect();
        Self { points, parents }
    }

    fn find(&self, mut point: usize) -> usize {
        while self.parents[point] != point {
            point = self.parents[point];
        }
        point
    }

    fn union(&mut self, point1: usize, point2: usize) {
        let root1 = self.find(point1);
        let root2 = self.find(point2);
        let min_root = root1.min(root2);
        self.parents[root1] = min_root;
        self.parents[root2] = min_root;
    }

    fn sizes(&self) -> Vec<usize> {
        let mut root_to_size = HashMap::with_capacity(self.points.len());
        for i in 0..self.points.len() {
            let root = self.find(i);
            root_to_size
                .entry(root)
                .and_modify(|size| *size += 1)
                .or_insert(1);
        }
        root_to_size.into_values().collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Distance {
    dist_sq: i64,
    index1: usize,
    index2: usize,
}

impl Distance {
    fn new(points: &Vec<Point>, index1: usize, index2: usize) -> Self {
        Self {
            dist_sq: points[index1].dist_sq(&points[index2]),
            index1,
            index2,
        }
    }
}

fn main() {
    let mut points = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            let mut coords = line.split(",");
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            let z = coords.next().unwrap().parse().unwrap();
            points.push(Point { x, y, z });
        }
    }

    let mut distances = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            distances.push(Reverse(Distance::new(&points, i, j)));
        }
    }

    let mut uf = UnionFind::new(points);
    let num_connections = if uf.points.len() < 50 { 10 } else { 1000 };
    for _ in 0..num_connections {
        let Reverse(Distance {
            dist_sq: _,
            index1,
            index2,
        }) = distances.pop().unwrap();
        uf.union(index1, index2);
    }
    let mut sizes = uf.sizes();
    sizes.sort();

    let mut size_product = 1;
    for i in 1..=3 {
        let size = sizes[sizes.len() - i];
        println!("Circuit of size {size}");
        size_product *= size;
    }

    println!("Product: {size_product}");
}

use std::{cmp::Ordering, io::BufRead};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn area(point1: &Point, point2: &Point) -> i64 {
    ((point1.x - point2.x).abs() + 1) * ((point1.y - point2.y).abs() + 1)
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Clockwise,
    Counterclockwise,
    Collinear,
}

impl Orientation {
    fn from_points(point1: &Point, point2: &Point, point3: &Point) -> Orientation {
        let slope1 = slope(point1, point2);
        let slope2 = slope(point2, point3);
        match slope1.cmp(&slope2) {
            Ordering::Equal => Orientation::Collinear,
            Ordering::Less => Orientation::Counterclockwise,
            Ordering::Greater => Orientation::Clockwise,
        }
    }
}

fn slope(point1: &Point, point2: &Point) -> i64 {
    (point2.y - point1.y) / (point2.x - point1.x)
}

fn intersects_1d(p1: i64, q1: i64, p2: i64, q2: i64) -> bool {
    let min1 = p1.min(q1);
    let max1 = p1.max(q1);
    let min2 = p2.min(q2);
    let max2 = p2.max(q2);

    match min1.cmp(&min2) {
        Ordering::Equal => true,
        Ordering::Less => max1 >= min2,
        Ordering::Greater => min1 <= max2,
    }
}

#[derive(Clone, Copy, Debug)]
struct LineSeg {
    point1: Point,
    point2: Point,
}

impl LineSeg {
    fn intersects(&self, other: &LineSeg) -> bool {
        let self1 = Orientation::from_points(&self.point1, &self.point2, &other.point1);
        let self2 = Orientation::from_points(&self.point1, &self.point2, &other.point2);
        let other1 = Orientation::from_points(&other.point1, &other.point2, &self.point1);
        let other2 = Orientation::from_points(&other.point1, &other.point2, &self.point2);

        if self1 != self2 && other1 != other2 {
            true
        } else if self1 == Orientation::Collinear
            && self2 == Orientation::Collinear
            && other1 == Orientation::Collinear
            && other2 == Orientation::Collinear
        {
            intersects_1d(self.point1.x, self.point2.x, other.point1.x, other.point2.x)
                && intersects_1d(self.point1.y, self.point2.y, other.point1.y, other.point2.y)
        } else {
            true
        }
    }
}

fn polygon_intersects_box(points: &Vec<Point>, point1: &Point, point2: &Point) -> bool {
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            // Check if any line in the polygon's boundary intersects with any of the four lines that defines our rectangle
        }
    }
    unimplemented!()
}

fn main() {
    let mut points = vec![];
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            let mut coords = line.split(',');
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();

            points.push(Point { x, y });
        }
    }

    let mut best_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let point1 = points[i];
            let point2 = points[j];

            if !polygon_intersects_box(&points, &point1, &point2) {
                best_area = best_area.max(area(&point1, &point2));
            }
        }
    }
    println!("Area: {best_area}");
}

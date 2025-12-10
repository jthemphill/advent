use std::io::BufRead;

struct Point {
    x: i64,
    y: i64,
}

fn area(point1: &Point, point2: &Point) -> i64 {
    ((point1.x - point2.x).abs() + 1) * ((point1.y - point2.y).abs() + 1)
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
            best_area = best_area.max(area(&points[i], &points[j]));
        }
    }
    println!("Area: {best_area}");
}

use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn area(point1: &Point, point2: &Point) -> i64 {
    ((point1.x - point2.x).abs() + 1) * ((point1.y - point2.y).abs() + 1)
}

fn is_legal(
    compressed_grid: &HashMap<(i64, i64), char>,
    compressed_x: &HashMap<i64, i64>,
    compressed_y: &HashMap<i64, i64>,
    point1: &Point,
    point2: &Point,
) -> bool {
    let &x1 = compressed_x.get(&point1.x).unwrap();
    let &x2 = compressed_x.get(&point2.x).unwrap();

    let xmin = x1.min(x2);
    let xmax = x1.max(x2);

    let &y1 = compressed_y.get(&point1.y).unwrap();
    let &y2 = compressed_y.get(&point2.y).unwrap();

    let ymin = y1.min(y2);
    let ymax = y1.max(y2);

    for x in xmin..xmax {
        for y in ymin..ymax {
            if compressed_grid.get(&(x, y)).is_none() {
                return false;
            }
        }
    }
    true
}

/**
 * Return a mapping of the given coordinates from their real values to a much smaller coordinate space
 */
fn compress(mut coordinates: Vec<i64>) -> HashMap<i64, i64> {
    let mut compressed = HashMap::new();

    coordinates.sort();
    for i in 0..coordinates.len() {
        compressed.insert(coordinates[i], i as i64);
    }
    compressed
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

    let compressed_x = compress(points.iter().map(|point| point.x).collect());
    let compressed_y = compress(points.iter().map(|point| point.y).collect());

    let mut compressed_grid = HashMap::new();
    for i in 0..points.len() {
        let point1 = points[i];

        let &x1 = compressed_x.get(&point1.x).unwrap();
        let &y1 = compressed_y.get(&point1.y).unwrap();

        // Insert a red tile at this point
        compressed_grid.insert((x1, y1), '#');

        // Get the next point in sequence
        let j = if i + 1 == points.len() { 0 } else { i + 1 };
        let point2 = points[j];

        let &x2 = compressed_x.get(&point2.x).unwrap();
        let &y2 = compressed_y.get(&point2.y).unwrap();

        // Insert green tiles between adjacent points
        if x1 == x2 {
            for y in y1.min(y2) + 1..y1.max(y2) {
                compressed_grid.insert((x1, y), 'X');
            }
        } else if y1 == y2 {
            for x in x1.min(x2) + 1..x1.max(x2) {
                compressed_grid.insert((x, y1), 'X');
            }
        } else {
            // Adjacent points were guaranteed to be in a straight line.
            panic!("{x1} != {x2} && {y1} != {y2}");
        }
    }

    // Print the shape before floodfill
    // for y in 0..=points.len() as i64 {
    //     let mut row = String::with_capacity(points.len());
    //     for x in 0..=points.len() as i64 {
    //         let c = *compressed_grid.get(&(x, y)).unwrap_or(&'.');
    //         row.push(c);
    //     }
    //     println!("{row}");
    // }
    // println!();

    // Fill the inside of the shape with green tiles
    // We do this by first collecting all points on the outside
    let mut outside = HashSet::new();
    let mut frontier = VecDeque::new();
    frontier.push_back((-1, -1));
    while let Some((x, y)) = frontier.pop_front() {
        // We've touched a red or green tile, so this is not a tile on the outside of the shape
        if compressed_grid.get(&(x, y)).is_some() {
            continue;
        }
        if !outside.insert((x, y)) {
            continue;
        }
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x2 = x + dx;
                let y2 = y + dy;
                if 0 <= x2 && x2 <= points.len() as i64 {
                    if 0 <= y2 && y2 <= points.len() as i64 {
                        frontier.push_back((x2, y2));
                    }
                }
            }
        }
    }

    // Fill in all the points that aren't on the outside
    for y in 0..=points.len() as i64 {
        for x in 0_i64..=points.len() as i64 {
            let c = *compressed_grid.get(&(x, y)).unwrap_or(&'.');
            if c == '.' && !outside.contains(&(x, y)) {
                compressed_grid.insert((x, y), 'X');
            }
        }
    }
    println!();

    // Print the shape
    // for y in 0..=points.len() as i64 {
    //     let mut row = String::with_capacity(points.len());
    //     for x in 0..=points.len() as i64 {
    //         let c = *compressed_grid.get(&(x, y)).unwrap_or(&'.');
    //         row.push(c);
    //     }
    //     println!("{row}");
    // }

    let mut best_area = 0;
    for i in 0..points.len() {
        let point1 = &points[i];
        for j in i + 1..points.len() {
            let point2 = &points[j];
            let rect_area = area(&point1, &point2);
            if rect_area > best_area
                && is_legal(
                    &compressed_grid,
                    &compressed_x,
                    &compressed_y,
                    &point1,
                    &point2,
                )
            {
                best_area = rect_area;
            }
        }
    }
    println!("Area: {best_area}");
}

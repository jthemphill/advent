use regex::Regex;
use std::collections::HashSet;

fn manhattan(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let re = Regex::new("Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)").unwrap();
    let mut beacons = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if let Some(caps) = re.captures(&line) {
                let sensor_x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let sensor_y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let nearest_x = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let nearest_y = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
                beacons.push((
                    (sensor_x, sensor_y),
                    (nearest_x, nearest_y),
                    manhattan((sensor_x, sensor_y), (nearest_x, nearest_y)),
                ));
            }
        }
    }
    let mut perimeters = HashSet::new();
    let min_dim = 0;
    let max_dim = 4_000_000;
    for &(sensor, _, dist) in &beacons {
        for x in (min_dim.max(sensor.0 - dist - 1))..(max_dim.min(sensor.0 + dist + 1)) {
            let x_dist = (sensor.0 - x).abs();
            let y_dist = (dist + 1) - x_dist;
            let y_low = sensor.1 - y_dist;
            if min_dim <= y_low && y_low <= max_dim {
                perimeters.insert((x, y_low));
            }
            let y_hi = sensor.1 + y_dist;
            if min_dim <= y_hi && y_hi <= max_dim {
                perimeters.insert((x, y_hi));
            }
        }
    }
    let remaining = perimeters
        .into_iter()
        .filter(|&p| {
            beacons
                .iter()
                .all(|&(sensor, _, dist)| manhattan(p, sensor) > dist)
        })
        .collect::<Vec<_>>();
    let distress = remaining[0];
    println!(
        "({}, {}): {}",
        distress.0,
        distress.1,
        distress.0 * max_dim + distress.1
    );
}

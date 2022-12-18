use regex::Regex;
use std::collections::HashSet;

fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let re = Regex::new("Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)").unwrap();
    let mut beacons = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if let Some(caps) = re.captures(&line) {
                let sensor_x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let sensor_y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let nearest_x = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let nearest_y = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                beacons.push(((sensor_x, sensor_y), (nearest_x, nearest_y)));
            }
        }
    }
    for impossible_y in [10, 2_000_000] {
        let mut impossible = HashSet::new();
        for &((sensor_x, sensor_y), (nearest_x, nearest_y)) in &beacons {
            let dist = manhattan((sensor_x, sensor_y), (nearest_x, nearest_y));
            for x in (sensor_x.min(nearest_x) - dist)..=(sensor_x.max(nearest_x) + dist) {
                if manhattan((sensor_x, sensor_y), (x, impossible_y)) <= dist {
                    impossible.insert(x);
                }
            }
        }
        for (_, (beacon_x, beacon_y)) in &beacons {
            if *beacon_y == impossible_y {
                impossible.remove(beacon_x);
            }
        }
        println!("y={}: {}", impossible_y, impossible.len());
    }
}

use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Target {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Probe {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Probe {
    pub fn new(velocity: (i32, i32)) -> Self {
        let position = (0, 0);
        Probe { position, velocity }
    }
}

fn step(probe: &Probe) -> Probe {
    let (mut x, mut y) = probe.position;
    let (mut dx, mut dy) = probe.velocity;

    x += dx;
    y += dy;

    if dx < 0 {
        dx += 1;
    } else if dx > 0 {
        dx -= 1;
    }

    dy -= 1;

    Probe {
        position: (x, y),
        velocity: (dx, dy),
    }
}

fn within_target(probe: &Probe, target: &Target) -> bool {
    let (x, y) = probe.position;
    target.min_x <= x && x <= target.max_x && target.min_y <= y && y <= target.max_y
}

fn parse_line(line: &str) -> Target {
    macro_rules! split {
        ($parts : expr, $split: expr) => {
            $parts.next().unwrap().splitn(2, $split)
        };
    }
    macro_rules! parse {
        ($parts : expr) => {
            $parts.next().unwrap().parse().unwrap()
        };
    }

    let mut parts = line.split("target area: x=");
    assert!(parts.next().unwrap().is_empty());

    let mut parts = split!(parts, "..");
    let min_x = parse!(parts);

    let mut parts = split!(parts, ", y=");
    let max_x = parse!(parts);
    let mut parts = split!(parts, "..");
    let min_y = parse!(parts);
    let max_y = parse!(parts);

    Target {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

fn get_highest_point(target: &Target, dx: i32, dy: i32) -> Option<i32> {
    let mut probe = Probe::new((dx, dy));
    let mut highest_point = 0;
    while target.min_y <= probe.position.1 || probe.velocity.1 > 0 {
        highest_point = highest_point.max(probe.position.1);
        if within_target(&probe, target) {
            return Some(highest_point);
        }
        probe = step(&probe);
    }
    None
}

fn main() {
    let target = parse_line(&std::io::stdin().lock().lines().next().unwrap().unwrap());
    let mut highest = 0;
    let mut num_working = 0;
    for dx in -500..500 {
        for dy in -500..500 {
            if let Some(high_point) = get_highest_point(&target, dx, dy) {
                highest = highest.max(high_point);
                num_working += 1;
            }
        }
    }
    println!("Highest: {}", highest);
    println!("Num working: {}", num_working);
}

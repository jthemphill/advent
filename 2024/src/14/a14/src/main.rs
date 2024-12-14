use std::io::Read;

use regex::Regex;

type Robot = ((i32, i32), (i32, i32));

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let re = Regex::new(r"p=(.*),(.*) v=(.*),(.*)").unwrap();

    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut robots: Vec<Robot> = vec![];
    for line in input.split('\n') {
        let captures = re.captures(line).unwrap();
        robots.push((
            (
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            ),
            (
                captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            ),
        ));
    }

    for _ in 0..100 {
        for ((px, py), (vx, vy)) in robots.iter_mut() {
            *px = (*px + *vx).rem_euclid(WIDTH);
            *py = (*py + *vy).rem_euclid(HEIGHT);
        }
    }

    let mut quad_a = 0;
    let mut quad_b = 0;
    let mut quad_c = 0;
    let mut quad_d = 0;

    for ((px, py), _) in robots {
        if px == (WIDTH - 1) / 2 || py == (HEIGHT - 1) / 2 {
            // Skip robots in the center
            continue;
        }

        let left = px < (WIDTH - 1) / 2;
        let top = py < (HEIGHT - 1) / 2;
        if top {
            if left {
                quad_a += 1;
            } else {
                quad_b += 1;
            }
        } else {
            if left {
                quad_c += 1;
            } else {
                quad_d += 1;
            }
        }
    }

    println!(
        "{quad_a} * {quad_b} * {quad_c} * {quad_d} = {}",
        quad_a * quad_b * quad_c * quad_d
    );
}

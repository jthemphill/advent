use std::io::Read;

use regex::Regex;

type Robot = ((i32, i32), (i32, i32));

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn display_robots(robots: &Vec<Robot>) {
    let mut grid: Vec<u8> = Vec::from([b'.'; HEIGHT as usize * (WIDTH as usize + 1)]);
    for y in 1..=HEIGHT as usize {
        grid[y * (WIDTH as usize + 1) - 1] = b'\n';
    }

    for ((px, py), _) in robots {
        assert!(0 <= *px && *px < WIDTH);
        assert!(0 <= *py && *py < HEIGHT);
        assert_ne!(
            grid[*py as usize * (WIDTH as usize + 1) + *px as usize],
            b'\n'
        );
        grid[*py as usize * (WIDTH as usize + 1) + *px as usize] = b'*';
    }
    let grid = String::from_utf8(grid).unwrap();
    println!("{grid}");
}

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

    let mut s = 0;
    println!("{} robots", robots.len());
    loop {
        let is_done = (|| {
            let mut quad_a = 0;
            let mut quad_b = 0;
            let mut quad_c = 0;
            let mut quad_d = 0;

            for &((px, py), _) in &robots {
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
            let threshold = robots.len() * 7 / 16;
            for quad in [quad_a, quad_b, quad_c, quad_d] {
                if quad > threshold {
                    return true;
                }
            }
            false
        })();
        if is_done {
            break;
        }

        for ((px, py), (vx, vy)) in robots.iter_mut() {
            *px = (*px + *vx).rem_euclid(WIDTH);
            *py = (*py + *vy).rem_euclid(HEIGHT);
        }

        s += 1;
    }
    println!("======= {s} SECONDS =======");
    display_robots(&robots);
}

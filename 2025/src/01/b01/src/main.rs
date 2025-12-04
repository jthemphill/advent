use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    magnitude: i32,
}

impl Rotation {
    fn from(line: &str) -> Self {
        let mut magnitude: i32 = 0;
        for num in line.as_bytes().into_iter().skip(1) {
            magnitude *= 10;
            magnitude += (num - b'0') as i32;
        }
        let dir_char = line.as_bytes()[0];
        let dir = match dir_char {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("First character: {dir_char}"),
        };
        Self { dir, magnitude }
    }
}

fn main() {
    let mut num_zeros = 0;
    let mut dial = 50;
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            let rotation = Rotation::from(&line);
            let mut incr_zeros = 0;

            let magnitude = if rotation.magnitude >= 100 {
                incr_zeros += rotation.magnitude / 100;
                rotation.magnitude % 100
            } else {
                rotation.magnitude
            };

            let mut next_dial = match rotation.dir {
                Direction::Left => dial - magnitude,
                Direction::Right => dial + magnitude,
            };
            if next_dial == 0 && dial != 0 {
                incr_zeros += 1;
            }
            if next_dial < 0 {
                next_dial += 100;
                if dial != 0 {
                    incr_zeros += 1;
                }
            }
            if next_dial >= 100 {
                next_dial -= 100;
                if dial != 0 {
                    incr_zeros += 1;
                }
            }
            dial = next_dial;
            num_zeros += incr_zeros;
            println!("Rotated {rotation:?}, pointing at {dial}, passed 0 {incr_zeros} times");
        }
    }
    println!("We pointed at 0 {num_zeros} times");
}

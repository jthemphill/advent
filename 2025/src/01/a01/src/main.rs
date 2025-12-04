use std::io::BufRead;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn from(line: &str) -> Self {
        let mut magnitude: i32 = 0;
        for num in line.as_bytes().into_iter().skip(1) {
            println!("{num}");
            magnitude *= 10;
            magnitude += (num - b'0') as i32;
        }
        let dir = line.as_bytes()[0];
        match dir {
            b'L' => Self::Left(magnitude),
            b'R' => Self::Right(magnitude),
            _ => panic!("First character: {dir}"),
        }
    }

    fn apply(&self, mut dial: i32) -> i32 {
        match self {
            Rotation::Left(magnitude) => dial -= magnitude,
            Rotation::Right(magnitude) => dial += magnitude,
        }
        while dial < 0 {
            dial += 100;
        }
        while dial > 99 {
            dial -= 100;
        }
        dial
    }
}

fn main() {
    let mut num_zeros = 0;
    let mut dial = 50;
    for line in std::io::stdin().lock().lines() {
        if let Ok(line) = line {
            let rotation = Rotation::from(&line);
            dial = rotation.apply(dial);
            if dial == 0 {
                num_zeros += 1;
            }
            println!("Rotated {rotation:?}, pointing at {dial}");
        }
    }
    println!("We pointed at 0 {num_zeros} times");
}

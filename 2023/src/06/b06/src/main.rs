#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn can_win(&self, button_time: usize) -> bool {
        assert!(button_time <= self.time);
        self.distance < button_time * (self.time - button_time)
    }

    fn ways_to_win(&self) -> usize {
        let mut total = 0;
        for button_time in 1..self.time {
            if self.can_win(button_time) {
                total += 1;
            }
        }
        total
    }
}

fn scrape_numbers(s: &str) -> usize {
    let mut total: usize = 0;
    for &char in s.as_bytes() {
        if b'0' <= char && char <= b'9' {
            total *= 10;
            total += (char - b'0') as usize;
        }
    }
    total
}

fn main() {
    let mut time = 0;
    let mut distance = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.starts_with("Time:") {
            time = scrape_numbers(line.split("Time:").nth(1).unwrap());
        }
        if line.starts_with("Distance:") {
            distance = scrape_numbers(line.split("Distance:").nth(1).unwrap());
        }
    }

    let race = Race { time, distance };
    println!("{:?}", race);
    println!("Ways: {}", race.ways_to_win());
}

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

fn main() {
    let mut times = vec![];
    let mut distances = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.starts_with("Time:") {
            times = line
                .split("Time:")
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|t| t.parse::<usize>().unwrap())
                .collect();
        }
        if line.starts_with("Distance:") {
            distances = line
                .split("Distance:")
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|t| t.parse::<usize>().unwrap())
                .collect();
        }
    }
    assert_eq!(times.len(), distances.len());
    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect();

    let mut combo_product = 1;
    for race in races {
        combo_product *= race.ways_to_win();
    }
    println!("Product: {}", combo_product);
}

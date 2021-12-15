fn main() {
    let mut fish = [0; 9];

    for line in include_str!("../input.txt").lines() {
        for num in line.split(',') {
            if let Ok(num) = num.parse::<usize>() {
                fish[num] += 1;
            }
        }
    }

    for _ in 1..=256 {
        let spawning = fish[0];
        for f in 1..=8 {
            fish[f - 1] = fish[f];
        }
        fish[6] += spawning;
        fish[8] = spawning;
        println!("{:?}", fish);
    }

    println!("{}", fish.iter().sum::<usize>());
}

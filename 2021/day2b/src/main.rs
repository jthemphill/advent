fn main() {
    let input = include_str!("../input.txt").lines().map(|line| {
        let mut split = line.split(" ");
        (
            split.next().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        )
    });

    let mut x = 0;
    let mut d = 0;
    let mut aim = 0;
    for (dir, mag) in input {
        match dir {
            "forward" => {
                x += mag;
                d += aim * mag;
            }
            "up" => aim -= mag,
            "down" => aim += mag,
            _ => panic!("Invalid direction: {}", dir),
        };
    }
    println!("{} * {} = {}", x, d, x * d);
}

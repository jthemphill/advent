fn main() {
    let mut x = 1;
    let mut t = 0;
    let mut signal_sum = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut split = line.split(" ");
            let cmd = split.next().unwrap();
            let (dt, dx) = match cmd {
                "noop" => (1, 0),
                "addx" => (2, split.next().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Unrecognized command: {}", cmd),
            };
            for _ in 0..dt {
                t += 1;
                if t % 20 == 0 {
                    let signal = t * x;
                    println!("t = {}, x = {}, signal = {}", t, x, signal);
                    match t {
                        20 | 60 | 100 | 140 | 180 | 220 => {
                            signal_sum += signal;
                        }
                        _ => {}
                    }
                }
            }
            x += dx;
        }
    }
    println!("Signal sum: {}", signal_sum);
}

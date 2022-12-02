fn main() {
    let mut my_score = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let opp = line.as_bytes()[0] - b'A';
            let strat = line.as_bytes()[2];

            my_score += match strat {
                // lose
                b'X' => 0 + (3 + opp - 1) % 3 + 1,
                // draw
                b'Y' => 3 + opp + 1,
                // win
                b'Z' => 6 + (opp + 1) % 3 + 1,
                _ => panic!("Invalid strat character: {}", strat),
            } as usize;
        }
    }
    println!("{}", my_score);
}

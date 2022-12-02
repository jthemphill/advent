fn main() {
    let mut my_score = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let opp = line.as_bytes()[0] - b'A';
            let me = line.as_bytes()[2] - b'X';

            my_score += (me + 1) as usize;

            if opp == me {
                my_score += 3;
            } else if (me + 3 - opp) % 3 == 1 {
                my_score += 6;
            }
        }
    }
    println!("{}", my_score);
}

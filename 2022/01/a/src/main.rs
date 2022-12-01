fn main() {
    let mut cur = 0;
    let mut best = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            if let Ok(i) = line.parse::<i32>() {
                cur += i;
            } else {
                if cur > best {
                    best = cur;
                }
                cur = 0;
            }
        }
    }
    println!("{}", best);
}

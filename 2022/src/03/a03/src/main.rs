fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let sz = line.as_bytes().len() / 2;
            let mut rucksack = line.as_bytes().iter().cloned();
            let mut first = std::collections::HashSet::new();
            for _ in 0..sz {
                first.insert(rucksack.next().unwrap());
            }
            for _ in 0..sz {
                let x = rucksack.next().unwrap();
                if first.contains(&x) {
                    println!("{}", x as char);
                    if b'a' <= x && x <= b'z' {
                        sum += (x - b'a' + 1) as usize;
                    } else if b'A' <= x && x <= b'Z' {
                        sum += (x - b'A' + 27) as usize;
                    }
                    break;
                }
            }
        }
    }
    println!("Total: {}", sum);
}

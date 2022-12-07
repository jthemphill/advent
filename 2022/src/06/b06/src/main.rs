const LEN: usize = 14;

fn main() {
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let buf = line.as_bytes();
            let ans = (LEN - 1..buf.len())
                .filter(|&i| (i + 1 - LEN..i).all(|a| (a + 1..=i).all(|b| buf[a] != buf[b])))
                .next()
                .unwrap();
            println!(
                "{}: {}",
                ans + 1,
                &String::from_iter(buf[ans + 1 - LEN..=ans].iter().map(|&c| c as char))
            );
        }
    }
}

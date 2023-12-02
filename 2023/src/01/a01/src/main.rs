fn main() {
    let mut total = 0;
    for line in std::io::stdin().lines() {
        macro_rules! get_digit {
            ($iter: expr) => {
                $iter
                    .find(|&&c| b'0' <= c && c <= b'9')
                    .map(|c| (c - b'0') as u32)
                    .unwrap();
            };
        }
        if let Ok(line) = line {
            let first_num = get_digit!(line.as_bytes().iter());
            let last_num = get_digit!(line.as_bytes().iter().rev());
            let ans = (first_num * 10) + last_num;
            println!("{}", ans);
            total += ans;
        }
    }
    println!("Total: {}", total);
}

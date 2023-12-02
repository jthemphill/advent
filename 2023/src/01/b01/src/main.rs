fn starts(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.len() >= needle.len() && haystack.iter().zip(needle.iter()).all(|(h, n)| h == n)
}

fn ends(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.len() >= needle.len()
        && haystack
            .iter()
            .rev()
            .zip(needle.iter().rev())
            .all(|(h, n)| h == n)
}

fn main() {
    let mut total = 0;
    for line_str in std::io::stdin().lines() {
        macro_rules! get_digit {
            ($iter: expr) => {
                $iter
                    .enumerate()
                    .find(|(_, &c)| b'0' <= c && c <= b'9')
                    .map(|(i, c)| (i, (c - b'0') as i32))
            };
        }
        if let Ok(line_str) = line_str {
            let line = line_str.as_bytes();
            let mut first_num = get_digit!(line.iter()).unwrap_or((line.len(), 0));
            let mut last_num = get_digit!(line.iter().rev())
                .map(|(i, c)| (line.len() - i, c))
                .unwrap_or((0, 0));
            println!("{} {:?} {:?}", line_str, first_num, last_num);
            let strs: [&[u8]; 10] = [
                b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight",
                b"nine",
            ];
            for (n, num_str) in strs.iter().enumerate() {
                for i in 0..first_num.0 {
                    if starts(&line[i..], num_str) {
                        first_num = (i, n as i32);
                        break;
                    }
                }
                for i in 0..(line.len() - last_num.0) {
                    if ends(&line[..line.len() - i], num_str) {
                        last_num = (line.len() - i, n as i32);
                        break;
                    }
                }
            }

            let ans = (first_num.1 * 10) + last_num.1;
            total += ans;
        }
    }
    println!("Total: {}", total);
}

#[test]
fn test_ends() {
    assert!(ends(b"asdf", b"df"));
    assert!(!ends(b"asd", b"df"));
}

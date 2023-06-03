#[derive(Debug, Copy, Clone)]
enum Digit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}

fn parse_snafu(bytes: &[u8]) -> Vec<Digit> {
    bytes
        .iter()
        .map(|byte| match byte {
            b'=' => Digit::DoubleMinus,
            b'-' => Digit::Minus,
            b'0' => Digit::Zero,
            b'1' => Digit::One,
            b'2' => Digit::Two,
            _ => panic!("Unrecognized character '{byte}'"),
        })
        .collect()
}

fn render_snafu(digits: &Vec<Digit>) -> Vec<u8> {
    digits
        .iter()
        .map(|digit| match digit {
            Digit::DoubleMinus => b'=',
            Digit::Minus => b'-',
            Digit::Zero => b'0',
            Digit::One => b'1',
            Digit::Two => b'2',
        })
        .collect()
}

fn snafu_to_int(digits: &Vec<Digit>) -> i64 {
    let mut sum = 0;
    let mut power = 1;
    for digit in digits.iter().rev() {
        sum += match digit {
            Digit::DoubleMinus => -2,
            Digit::Minus => -1,
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
        } * power;
        power *= 5;
    }
    sum
}

fn int_to_snafu(mut num: i64) -> Vec<Digit> {
    let mut digits = vec![];
    while num != 0 {
        let remainder = num % 5;
        digits.push(match remainder {
            0 => Digit::Zero,
            1 => {
                num -= 1;
                Digit::One
            }
            2 => {
                num -= 2;
                Digit::Two
            }
            3 => {
                num += 2;
                Digit::DoubleMinus
            }
            4 => {
                num += 1;
                Digit::Minus
            }
            _ => panic!("Modulo 5"),
        });
        num /= 5;
    }
    digits.reverse();
    digits
}

#[test]
fn test_examples() {
    for (num, str) in [
        (2022, b"1=11-2" as &[u8]),
        (12345, b"1-0---0" as &[u8]),
        (314159265, b"1121-1110-1=0" as &[u8]),
    ] {
        assert_eq!(snafu_to_int(&parse_snafu(str)), num);
        assert_eq!(render_snafu(&int_to_snafu(num)), str);
    }
}

fn main() {
    let mut sum = 0;
    for line in std::io::BufRead::lines(std::io::stdin().lock()) {
        sum += snafu_to_int(&parse_snafu(line.unwrap().as_bytes()));
    }
    println!(
        "{}",
        String::from_utf8(render_snafu(&int_to_snafu(sum))).unwrap()
    );
}

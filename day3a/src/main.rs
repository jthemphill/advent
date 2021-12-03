fn main() {
    let nums = include_str!("../input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    let bitlen = nums[0].len();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..bitlen {
        let mut count = 0;
        for num in &nums {
            match num[i] as char {
                '0' => count -= 1,
                '1' => count += 1,
                _ => panic!("Unexpected string {:?}", num),
            }
        }
        if count > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma_dec = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("{}", gamma_dec * epsilon_dec);
}

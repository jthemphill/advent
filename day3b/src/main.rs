fn oxygen(mut nums: Vec<&[u8]>) -> &str {
    let bitlen = nums[0].len();
    for i in 0..bitlen {
        if nums.len() == 1 {
            break;
        }
        let mut count = 0;
        for num in &nums {
            match num[i] as char {
                '0' => count -= 1,
                '1' => count += 1,
                _ => panic!("Unexpected string {:?}", num),
            }
        }
        if count >= 0 {
            nums = nums.into_iter().filter(|n| n[i] as char == '1').collect();
        } else {
            nums = nums.into_iter().filter(|n| n[i] as char == '0').collect();
        }
    }
    std::str::from_utf8(nums[0]).unwrap()
}

fn co2(mut nums: Vec<&[u8]>) -> &str {
    let bitlen = nums[0].len();
    for i in 0..bitlen {
        if nums.len() == 1 {
            break;
        }
        let mut count = 0;
        for num in &nums {
            match num[i] as char {
                '0' => count -= 1,
                '1' => count += 1,
                _ => panic!("Unexpected string {:?}", num),
            }
        }
        if count < 0 {
            nums = nums.into_iter().filter(|n| n[i] as char == '1').collect();
        } else {
            nums = nums.into_iter().filter(|n| n[i] as char == '0').collect();
        }
    }
    std::str::from_utf8(nums[0]).unwrap()
}

fn main() {
    let nums = include_str!("../input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    let o2_dec = isize::from_str_radix(oxygen(nums.clone()), 2).unwrap();
    let co2_dec = isize::from_str_radix(co2(nums), 2).unwrap();
    println!(
        "O2: {}, CO2: {}, Ans: {}",
        o2_dec,
        co2_dec,
        o2_dec * co2_dec
    );
}

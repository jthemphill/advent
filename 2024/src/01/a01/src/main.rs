fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut nums = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    left.sort();
    right.sort();

    let mut total_distance = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        total_distance += (r - l).abs();
    }
    println!("{}", total_distance);
}

use regex::Regex;

fn main() {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        for capture in mul_re.captures_iter(&line) {
            let lhs: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
            let rhs: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
            total += lhs * rhs;
        }
    }
    println!("{total}");
}

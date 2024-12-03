use regex::Regex;
use std::io::Read;

fn mul_total(s: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for capture in mul_re.captures_iter(&s) {
        let lhs: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
        let rhs: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
        total += lhs * rhs;
    }
    total
}

fn main() {
    let dont_re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let dont_end = Regex::new(r"don't\(\).*").unwrap();

    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let total = mul_total(&input);
    println!("Unprocessed:        {total}");

    let input = input.replace("\n", " ");
    let total = mul_total(&input);
    println!("Newlines removed:   {total}");

    let input = dont_re.replace_all(&input, "XXXXXXXXXX").into_owned();
    let total = mul_total(&input);
    println!("Mid don'ts removed: {total}");

    let input = dont_end.replace_all(&input, "ZZZZZZZZZZ").into_owned();
    let total = mul_total(&input);
    println!("Last don't removed: {total}");
}

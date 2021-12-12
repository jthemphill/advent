fn main() {
    let mut count = 0;
    let mut last = 0;
    for num in include_str!("../input.txt").lines().map(|i| i.parse::<usize>().unwrap()) {
        if num > last {
            count += 1;
        }
        last = num;
    }
    println!("{}", count - 1);
}

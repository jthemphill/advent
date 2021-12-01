fn main() {
    let mut window = std::collections::VecDeque::<usize>::with_capacity(4);
    let mut count = 0;
    for num in include_str!("../input.txt").lines().map(|i| i.parse::<usize>().unwrap()) {
        window.push_back(num);
        if window.len() == 4 {
            if window[0] < window[3] {
                count += 1;
            }
            window.pop_front();
        }
    }
    println!("{}", count);
}

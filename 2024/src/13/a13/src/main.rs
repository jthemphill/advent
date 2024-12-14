use std::io::Read;

use regex::Regex;

#[derive(Clone, Copy, Debug, Default)]
struct Puzzle {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

fn div(a: i32, b: i32) -> Option<i32> {
    if a % b == 0 {
        Some(a / b)
    } else {
        None
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let button_a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut puzzles: Vec<Puzzle> = vec![];

    let mut puzzle = Puzzle::default();
    for line in input.split('\n') {
        if let Some(button_a) = button_a_re.captures(line) {
            puzzle.a = (
                button_a.get(1).unwrap().as_str().parse().unwrap(),
                button_a.get(2).unwrap().as_str().parse().unwrap(),
            );
        } else if let Some(button_b) = button_b_re.captures(line) {
            puzzle.b = (
                button_b.get(1).unwrap().as_str().parse().unwrap(),
                button_b.get(2).unwrap().as_str().parse().unwrap(),
            );
        } else if let Some(prize) = prize_re.captures(line) {
            puzzle.prize = (
                prize.get(1).unwrap().as_str().parse().unwrap(),
                prize.get(2).unwrap().as_str().parse().unwrap(),
            );
            let mut new_puzzle = Puzzle::default();
            std::mem::swap(&mut puzzle, &mut new_puzzle);
            puzzles.push(new_puzzle);
        }
    }

    let mut total_cost = 0;
    for puzzle in puzzles {
        // 94a + 22b = 8400 X
        // 34a + 67b = 5400 Y

        // b = (5400 - 34a) / 67
        // 94a + 22(5400 - 34a)/67 = 8400
        // 94*67a + 22*5400 - 22*34a = 8400 * 67
        // a(94*67 - 22*34) = 8400 * 67 - 22*5400
        // a = (8400 * 67 - (22 * 5400)) / (94*67 - 22 * 34)
        // a = ( X   * by - (bx *  Y  )) / (ax*by - bx * ay)

        let (ax, ay) = puzzle.a;
        let (bx, by) = puzzle.b;
        let (x, y) = puzzle.prize;

        if let Some(a) = div(x * by - (bx * y), ax * by - bx * ay) {
            if let Some(b) = div(x - (ax * a), bx) {
                let cost = 3 * a + b;
                total_cost += cost;
            }
        }
    }
    println!("Total cost: {total_cost}");
}

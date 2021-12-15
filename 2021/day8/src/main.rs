use std::collections::HashMap;

const A_SCORE: i64 = 8;
const B_SCORE: i64 = 6;
const C_SCORE: i64 = 8;
const D_SCORE: i64 = 7;
const E_SCORE: i64 = 4;
const F_SCORE: i64 = 9;
const G_SCORE: i64 = 7;

const ZERO_SCORE: i64 = A_SCORE + B_SCORE + C_SCORE + E_SCORE + F_SCORE + G_SCORE;
const ONE_SCORE: i64 = C_SCORE + F_SCORE;
const TWO_SCORE: i64 = A_SCORE + C_SCORE + D_SCORE + E_SCORE + G_SCORE;
const THREE_SCORE: i64 = A_SCORE + C_SCORE + D_SCORE + F_SCORE + G_SCORE;
const FOUR_SCORE: i64 = B_SCORE + C_SCORE + D_SCORE + F_SCORE;
const FIVE_SCORE: i64 = A_SCORE + B_SCORE + D_SCORE + F_SCORE + G_SCORE;
const SIX_SCORE: i64 = A_SCORE + B_SCORE + D_SCORE + E_SCORE + F_SCORE + G_SCORE;
const SEVEN_SCORE: i64 = A_SCORE + C_SCORE + F_SCORE;
const EIGHT_SCORE: i64 = A_SCORE + B_SCORE + C_SCORE + D_SCORE + E_SCORE + F_SCORE + G_SCORE;
const NINE_SCORE: i64 = A_SCORE + B_SCORE + C_SCORE + D_SCORE + F_SCORE + G_SCORE;

fn main() {
    println!(
        "Digit scores. 0: {}, 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}, 8: {}, 9: {}",
        ZERO_SCORE,
        ONE_SCORE,
        TWO_SCORE,
        THREE_SCORE,
        FOUR_SCORE,
        FIVE_SCORE,
        SIX_SCORE,
        SEVEN_SCORE,
        EIGHT_SCORE,
        NINE_SCORE
    );

    let mut total_output = 0;
    for line in include_str!("../input.txt").lines() {
        let mut letter_counts: HashMap<char, i64> = HashMap::new();
        let mut parts = line.split(" | ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        for tok in left.split(' ') {
            for c in tok.chars() {
                *letter_counts.entry(c).or_insert(0) += 1;
            }
        }
        let mut output = 0;
        for tok in right.split(' ') {
            let digit_score = tok.chars().map(|c| letter_counts.get(&c).unwrap()).sum();
            output *= 10;
            output += match digit_score {
                ZERO_SCORE => 0,
                ONE_SCORE => 1,
                TWO_SCORE => 2,
                THREE_SCORE => 3,
                FOUR_SCORE => 4,
                FIVE_SCORE => 5,
                SIX_SCORE => 6,
                SEVEN_SCORE => 7,
                EIGHT_SCORE => 8,
                NINE_SCORE => 9,
                _ => panic!(
                    "Never saw a digit score of {} before for {}",
                    digit_score, tok,
                ),
            };
        }
        println!("Output: {}", output);
        total_output += output;
    }
    println!("Total output: {}", total_output);
}

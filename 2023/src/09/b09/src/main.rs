fn difference_seq(seq: &[i64]) -> Vec<i64> {
    let mut diff_seq = vec![];
    for i in 1..seq.len() {
        diff_seq.push(seq[i] - seq[i - 1]);
    }
    diff_seq
}

fn main() {
    let mut sequences: Vec<Vec<i64>> = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        sequences.push(
            line.split_ascii_whitespace()
                .map(|token| token.parse().unwrap())
                .collect(),
        );
    }

    let mut extra_sum = 0;
    for seq in sequences {
        let mut seq_stack = vec![seq];
        while seq_stack.last().unwrap().iter().any(|&n| n != 0) {
            seq_stack.push(difference_seq(seq_stack.last().unwrap()));
        }

        let mut first_val_now = 0;
        for i in (0..(seq_stack.len() - 1)).rev() {
            first_val_now = *seq_stack[i].first().unwrap() - first_val_now;
        }
        extra_sum += first_val_now;
    }
    println!("Sum of extrapolated values: {extra_sum}");
}
